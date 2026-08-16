//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 447/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk447(t3031: f64, t3032: f64, t371: f64, t335: f64, t368: f64, t1015: f64, t1022: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3033 = t3031 * t3032;
    let t3034 = t371 * t371;
    let t3036 = 1.0_f64 / t3034 / t335;
    let t3037 = t368 * t3036;
    let t3038 = t1015 * t3037;
    let t3039 = t3033 * t3038;
    let t3040 = t1022 * t1022;
    (t3033, t3034, t3036, t3037, t3039, t3040)
}
