//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 926/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk926(t18483: f64, t19247: f64, t5773: f64, t125: f64, t4516: f64, t5833: f64, t506: f64, t5832: f64, t1509: f64, t7236: f64, t486: f64, t7271: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64) {
    let t19249 = 0.16239027777777777777e1_f64 * param_hyb_omega_0 * t18483 * t5773 * t19247;
    let t19316 = 0.16322666666666666667e0_f64 * t125 * t4516 * t5833 * t19247;
    let t19342 = t5832 * t506;
    let t19349 = 0.57738765432098765432e1_f64 * t1509 * t7236;
    let t19351 = 0.50521419753086419753e1_f64 * t486 * t7271;
    (t19249, t19316, t19342, t19349, t19351)
}
