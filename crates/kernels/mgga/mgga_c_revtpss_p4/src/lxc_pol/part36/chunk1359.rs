//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1359/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1359<F: Float>(t355: F, t91338: F, t1769: F, t471: F, t111865: F, t112880: F, t1287: F, t13129: F, t1775: F, t1794: F, t1828: F, t24543: F, t24906: F, t26889: F, t26895: F, t26969: F, t26976: F, t29141: F, t29207: F, t30735: F, t30751: F, t30763: F, t30768: F, t30771: F, t30840: F, t30870: F, t6703: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t7660: F, t8202: F, t8217: F, t96888: F, t96927: F, t96953: F) -> (F, F) {
    let t116356 = t355 * t91338;
    let t116360 = t471 * t1769;
    let t116381 = -F::cast_from(0.19756347548806534796e1_f64) * t111865 * t1775 + F::cast_from(0.52041769129231196772e1_f64) * t7636 * t7652 * t30751 * t1828 - F::cast_from(0.39512695097613069591e1_f64) * t26976 * t24906 - F::cast_from(0.52041769129231196772e1_f64) * t7643 * t7652 * t30735 * t1828 + F::cast_from(0.26020884564615598386e1_f64) * t26895 * t30735 * t1794 * t1287 - F::cast_from(0.26020884564615598386e1_f64) * t26889 * t30751 * t1794 * t1287 + F::cast_from(0.39512695097613069591e1_f64) * t29207 * t6703 + F::cast_from(0.26020884564615598386e1_f64) * t112880 * t8202 - F::cast_from(0.10408353825846239354e2_f64) * t96927 * t30763 * t116356 + F::cast_from(0.10408353825846239354e2_f64) * t96953 * t30763 * t355 * t116360 - F::cast_from(0.78062653693846795158e1_f64) * t29141 * t30768 - F::cast_from(0.78062653693846795158e1_f64) * t7651 * t26969 * t30771 * t1828 - F::cast_from(0.4336814094102599731e0_f64) * t96888 * t7660 * t24543 * t13129 - F::cast_from(0.13010442282307799193e1_f64) * t30870 * t8217 - F::cast_from(0.26020884564615598386e1_f64) * t7636 * t7637 * t30840 * t1769;
    (t116356, t116381)
}
