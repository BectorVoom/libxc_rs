//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1985/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1985(t92586: f64, t92605: f64, t92623: f64, t92642: f64, t92663: f64, t92682: f64, t92701: f64, t92719: f64, t87565: f64, t226: f64, t235: f64, t24269: f64, t26661: f64, t2684: f64, t4234: f64, t812: f64, t81623: f64, t81630: f64, t81633: f64, t81642: f64, t81653: f64, t87531: f64, t87538: f64, t87541: f64, t87554: f64, t92560: f64, t92561: f64, t92564: f64, t92565: f64) -> (f64, f64) {
    let t92722 = t92586 + t92605 + t92623 + t92642 + t92663 + t92682 + t92701 + t92719;
    let t92729 = 0.15352717957250113407e0_f64 * t87565;
    let t92732 = -0.13159472534785811492e0_f64 * t87531 + t92560 + t92561 - 0.16449340668482264365e-1_f64 * t87538 + 0.6579736267392905746e-1_f64 * t87541 - t92564 - t92565 - 0.3289868133696452873e-1_f64 * t87554 - 2.0_f64 * t812 * t24269 * t4234 + 0.15352717957250113407e0_f64 * t81623 + t226 * t235 * t92722 + 0.16449340668482264365e-1_f64 * t81630 - 0.51175726524167044691e0_f64 * t81633 - 0.49348022005446793095e-1_f64 * t81642 - 0.3289868133696452873e-1_f64 * t81653 - t92729 - t812 * t26661 * t2684;
    (t92722, t92732)
}
