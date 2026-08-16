//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1322/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1322(t13196: f64, t2701: f64, t820: f64, t2563: f64, t4159: f64, t119: f64, t12971: f64, t210: f64, t4155: f64, t9573: f64, t2645: f64, t2684: f64, t4248: f64) -> (f64, f64, f64, f64, f64) {
    let t13198 = t2701 * t820 * t13196;
    let t13202 = 7.0_f64 / 72.0_f64 * t2563 * t4159;
    let t13203 = t119 * t12971;
    let t13204 = t210 * t13203;
    let t13208 = 7.0_f64 / 24.0_f64 * t9573 * t4155;
    let t13210 = t2645 * t4248 * t2684;
    (t13198, t13202, t13204, t13208, t13210)
}
