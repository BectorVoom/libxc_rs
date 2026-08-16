//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1215/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1215(t13519: f64, t13521: f64, t13525: f64, t13527: f64, t13529: f64, t13530: f64, t13668: f64, t13675: f64, t13676: f64, t13677: f64, t13678: f64, t13682: f64, t13684: f64, t13687: f64, t13689: f64, t13693: f64, t13697: f64, t13699: f64, t13703: f64, t13707: f64, t13709: f64, t13711: f64, t13714: f64) -> (f64, f64) {
    let t14436 = t13519 + t13521 + t13525 - t13527 - t13529 - t13530 - t13668 - t13675 - t13676 - t13677 + t13678;
    let t14437 = -t13682 - t13684 - t13687 - t13689 - t13693 - t13697 - t13699 - t13703 + t13707 - t13709 - t13711 - t13714;
    (t14436, t14437)
}
