//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1043/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1043(t2250: f64, t31682: f64, t8308: f64, t131: f64, t2240: f64, t23966: f64, t31684: f64, t31680: f64, t9231: f64, t8511: f64, t9239: f64, t1862: f64, t645: f64) -> (f64, f64, f64, f64, f64) {
    let t115884 = t8308 * t31682 * t2250;
    let t115888 = t2240 * t23966 * t131;
    let t115889 = t115888 * t31684;
    let t115891 = t9231 * t31680;
    let t115894 = t8511 * t131;
    let t115895 = t9239 * t115894;
    let t115896 = t1862 * t645;
    (t115884, t115889, t115891, t115895, t115896)
}
