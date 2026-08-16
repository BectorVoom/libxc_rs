//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1156/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1156(t12723: f64, t7130: f64, t41133: f64, t5211: f64, t7491: f64, t954: f64, t12782: f64, t7115: f64, t7117: f64, t42142: f64, t33281: f64, t184: f64, t221: f64, t3477: f64, t3491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48373 = 32.0_f64 / 15.0_f64 * t7130 * t12723;
    let t48377 = 32.0_f64 / 9.0_f64 * t5211 * t7491 * t41133 * t954;
    let t48380 = 32.0_f64 / 15.0_f64 * t7115 * t7117 * t12782;
    let t48381 = 16.0_f64 / 15.0_f64 * t42142;
    let t48382 = 8.0_f64 / 45.0_f64 * t33281;
    let t48387 = 8.0_f64 / 5.0_f64 * t3491 * t3477 * t184 * t221;
    (t48373, t48377, t48380, t48381, t48382, t48387)
}
