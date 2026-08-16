//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1303/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1303(t1276: f64, t2148: f64, t3140: f64, t3727: f64, t12630: f64, t1294: f64, t13043: f64, t13129: f64, t13143: f64, t13149: f64, t13174: f64, t26907: f64, t26909: f64, t26913: f64, t26969: f64, t26979: f64, t26987: f64, t27011: f64, t27015: f64, t3569: f64, t3576: f64, t3585: f64, t7602: f64, t7645: f64, t7651: f64, t7654: f64, t7660: f64, t7662: f64, t96861: f64, t96866: f64, t96870: f64, t96874: f64, t96883: f64, t96888: f64, t96889: f64) -> f64 {
    let t96910 = t2148 * t3727 * t3140 * t1276;
    let t96913 = -0.39512695097613069591e1_f64 * t7602 * t13174 - 0.39512695097613069591e1_f64 * t96861 * t12630 - 0.10408353825846239354e2_f64 * t26979 * t27015 + 0.39512695097613069591e1_f64 * t96866 * t3569 + 0.52041769129231196772e1_f64 * t96870 * t7645 + 0.26020884564615598386e1_f64 * t96874 * t7654 - 0.19756347548806534796e1_f64 * t27011 * t3585 + 0.39512695097613069591e1_f64 * t27011 * t3576 - 0.26020884564615598386e1_f64 * t96883 * t26909 - 0.26020884564615598386e1_f64 * t96888 * t96889 * t13043 * t13149 + 0.26020884564615598386e1_f64 * t96888 * t26907 * t13043 * t13143 + 0.13010442282307799193e1_f64 * t96883 * t26913 - 0.4336814094102599731e0_f64 * t96888 * t7660 * t13043 * t13129 - 0.78062653693846795158e1_f64 * t7651 * t26969 * t26987 * t1294 - 0.13010442282307799193e1_f64 * t96910 * t7662;
    t96913
}
