//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1315/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1315(t11239: f64, t8316: f64, t11243: f64, t8493: f64, t190: f64, t5589: f64, t674: f64, t8451: f64, t11395: f64, t5: f64, t25708: f64, t4055: f64, t8452: f64) -> (f64, f64, f64, f64, f64) {
    let t35674 = t8316 * t11239;
    let t35676 = t8493 * t11243;
    let t35680 = t8451 * t190 * t674 * t5589;
    let t35682 = t5 * t11395;
    let t35685 = t35682 * t25708 * t8452 * t4055;
    (t35674, t35676, t35680, t35682, t35685)
}
