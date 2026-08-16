//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 836/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk836(t4300: f64, t1342: f64, t1339: f64, t3512: f64, t3754: f64, t1390: f64, t313: f64, t1336: f64, t140: f64, t3531: f64, t441: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12817 = t4300 * sigma0;
    let t12818 = t12817 * t1342;
    let t12819 = t1339 * t12818;
    let t12821 = t3512 * t3754;
    let t12822 = t1339 * t12821;
    let t12825 = 1.0_f64 / t313 / t1390;
    let t12827 = t140 * t1336 * t12825;
    let t12829 = 1.0_f64 / t3531 / t441;
    (t12817, t12819, t12822, t12825, t12827, t12829)
}
