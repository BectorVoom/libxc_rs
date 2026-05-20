//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2649/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2649<F: Float>(t46917: F, t5706: F, t241: F, t47201: F, t820: F, t46478: F, t9898: F, t13783: F, t13804: F, t13926: F, t13975: F, t3924: F, t3934: F, t3936: F, t46812: F, t46817: F, t46820: F, t46824: F, t46828: F, t46831: F, t46833: F, t46837: F, t46840: F, t46846: F, t46853: F, t46859: F, t47248: F, t47249: F, t5673: F, t5674: F, t9956: F, t9995: F) -> (F, F) {
    let t48756 = t46917 * t5706;
    let t48759 = t820 * t47201 * t241;
    let t48760 = t46478 * t9898;
    let t48778 = -F::cast_from(0.13553694749236397037e-4_f64) * t46812 - t46817 + t46820 - t46824 + F::cast_from(0.1084295579938911763e-3_f64) * t46828 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t13975 * t3924 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t13926 * t9956 - t46831 + F::cast_from(0.68026775414003982662e-1_f64) * t48756 + F::cast_from(0.51448821741683684368e-2_f64) * t48759 * t5673 * t5674 * t48760 - F::cast_from(0.77173232612525526552e-2_f64) * t13804 * t5673 * t5674 * t9995 + F::cast_from(0.25724410870841842183e-1_f64) * t3934 * t47248 * t5674 * t47249 + F::cast_from(0.24396650548625514668e-3_f64) * t46833 - F::cast_from(0.6098400337114239387e-4_f64) * t46837 + t46840 - F::cast_from(0.60023625365297631762e-1_f64) * t46846 + F::cast_from(0.76230004213927992336e-4_f64) * t46853 - F::cast_from(0.60246173160355784832e-6_f64) * t46859;
    (t48760, t48778)
}
