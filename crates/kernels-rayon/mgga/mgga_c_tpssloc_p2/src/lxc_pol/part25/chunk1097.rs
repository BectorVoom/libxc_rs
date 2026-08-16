//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1097/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1097(t12248: f64, t59: f64, t1336: f64, t240: f64, t12293: f64, t12297: f64, t22761: f64, t12305: f64, t6952: f64, t12267: f64, t6944: f64, t1354: f64) -> (f64, f64, f64, f64) {
    let t80901 = t12248 * t59;
    let t80903 = t1336 * t80901 * t240;
    let t80904 = t80903 * t12293;
    let t80906 = t22761 * t12297;
    let t80908 = t6952 * t12305;
    let t80910 = t12267 * t6944;
    let t80911 = t80910 * t1354;
    (t80904, t80906, t80908, t80911)
}
