//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 819/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk819(t7321: f64, t7322: f64, t2634: f64, t495: f64, t5109: f64, t2654: f64, t1568: f64, t2123: f64, t1569: f64, t920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7323 = t7321 * t7322;
    let t7326 = t2634 * t495;
    let t7327 = t5109 * t7326;
    let t7330 = t5109 * t7322;
    let t7333 = t2654 * t495;
    let t7334 = t5109 * t7333;
    let t7337 = t2123 * t1568;
    let t7338 = t920 * t1569;
    (t7323, t7326, t7327, t7330, t7333, t7334, t7337, t7338)
}
