//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1359/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1359(t355: f64, t91338: f64, t1769: f64, t471: f64, t111865: f64, t112880: f64, t1287: f64, t13129: f64, t1775: f64, t1794: f64, t1828: f64, t24543: f64, t24906: f64, t26889: f64, t26895: f64, t26969: f64, t26976: f64, t29141: f64, t29207: f64, t30735: f64, t30751: f64, t30763: f64, t30768: f64, t30771: f64, t30840: f64, t30870: f64, t6703: f64, t7636: f64, t7637: f64, t7643: f64, t7651: f64, t7652: f64, t7660: f64, t8202: f64, t8217: f64, t96888: f64, t96927: f64, t96953: f64) -> (f64, f64) {
    let t116356 = t355 * t91338;
    let t116360 = t471 * t1769;
    let t116381 = -0.19756347548806534796e1_f64 * t111865 * t1775 + 0.52041769129231196772e1_f64 * t7636 * t7652 * t30751 * t1828 - 0.39512695097613069591e1_f64 * t26976 * t24906 - 0.52041769129231196772e1_f64 * t7643 * t7652 * t30735 * t1828 + 0.26020884564615598386e1_f64 * t26895 * t30735 * t1794 * t1287 - 0.26020884564615598386e1_f64 * t26889 * t30751 * t1794 * t1287 + 0.39512695097613069591e1_f64 * t29207 * t6703 + 0.26020884564615598386e1_f64 * t112880 * t8202 - 0.10408353825846239354e2_f64 * t96927 * t30763 * t116356 + 0.10408353825846239354e2_f64 * t96953 * t30763 * t355 * t116360 - 0.78062653693846795158e1_f64 * t29141 * t30768 - 0.78062653693846795158e1_f64 * t7651 * t26969 * t30771 * t1828 - 0.4336814094102599731e0_f64 * t96888 * t7660 * t24543 * t13129 - 0.13010442282307799193e1_f64 * t30870 * t8217 - 0.26020884564615598386e1_f64 * t7636 * t7637 * t30840 * t1769;
    (t116356, t116381)
}
