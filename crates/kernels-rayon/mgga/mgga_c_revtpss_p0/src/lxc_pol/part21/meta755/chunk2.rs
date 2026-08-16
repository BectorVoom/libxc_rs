//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2649/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2649(t46917: f64, t5706: f64, t241: f64, t47201: f64, t820: f64, t46478: f64, t9898: f64, t13783: f64, t13804: f64, t13926: f64, t13975: f64, t3924: f64, t3934: f64, t3936: f64, t46812: f64, t46817: f64, t46820: f64, t46824: f64, t46828: f64, t46831: f64, t46833: f64, t46837: f64, t46840: f64, t46846: f64, t46853: f64, t46859: f64, t47248: f64, t47249: f64, t5673: f64, t5674: f64, t9956: f64, t9995: f64) -> (f64, f64) {
    let t48756 = t46917 * t5706;
    let t48759 = t820 * t47201 * t241;
    let t48760 = t46478 * t9898;
    let t48778 = -0.13553694749236397037e-4_f64 * t46812 - t46817 + t46820 - t46824 + 0.1084295579938911763e-3_f64 * t46828 + 0.25724410870841842183e-2_f64 * t3934 * t3936 * t13975 * t3924 - 0.12862205435420921092e-1_f64 * t3934 * t13783 * t13926 * t9956 - t46831 + 0.68026775414003982662e-1_f64 * t48756 + 0.51448821741683684368e-2_f64 * t48759 * t5673 * t5674 * t48760 - 0.77173232612525526552e-2_f64 * t13804 * t5673 * t5674 * t9995 + 0.25724410870841842183e-1_f64 * t3934 * t47248 * t5674 * t47249 + 0.24396650548625514668e-3_f64 * t46833 - 0.6098400337114239387e-4_f64 * t46837 + t46840 - 0.60023625365297631762e-1_f64 * t46846 + 0.76230004213927992336e-4_f64 * t46853 - 0.60246173160355784832e-6_f64 * t46859;
    (t48760, t48778)
}
