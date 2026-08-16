//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1363/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1363(t2924: f64, t2932: f64, t2860: f64, t919: f64, t2904: f64, t938: f64, t10629: f64, t315: f64, t2853: f64, t923: f64, t2885: f64, t2884: f64, t307: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10723 = t2924 * t2932;
    let t10740 = t919 * t2860;
    let t10747 = t938 * t2904;
    let t10756 = t315 * t10629;
    let t10760 = t2853 * t923;
    let t10765 = t919 * t2885;
    let t10770 = 1.0_f64 / t2884 / t307;
    (t10723, t10740, t10747, t10756, t10760, t10765, t10770)
}
