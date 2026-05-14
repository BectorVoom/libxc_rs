//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1146/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1146<F: Float>(t2148: F, t96873: F, t11239: F, t1269: F, t1276: F, t42859: F, t487: F, t13038: F, t2142: F, t3140: F, t3727: F, t12630: F, t1294: F, t13043: F, t13129: F, t13143: F, t13149: F, t13174: F, t26907: F, t26909: F, t26913: F, t26969: F, t26979: F, t26987: F, t27011: F, t27015: F, t3569: F, t3576: F, t3585: F, t7602: F, t7645: F, t7651: F, t7654: F, t7660: F, t7662: F, t96861: F, t96866: F, t96870: F) -> (F,) {
    let t96874 = t2148 * t96873;
    let t96881 = t1269 * t11239;
    let t96883 = t2148 * t96881 * t1276;
    let t96886 = t487 * t42859;
    let t96888 = t2148 * t96886 * t1276;
    let t96889 = t13038 * t2142;
    let t96910 = t2148 * t3727 * t3140 * t1276;
    let t96913 = -0.39512695097613069591e1 * t7602 * t13174 - 0.39512695097613069591e1 * t96861 * t12630 - 0.10408353825846239354e2 * t26979 * t27015 + 0.39512695097613069591e1 * t96866 * t3569 + 0.52041769129231196772e1 * t96870 * t7645 + 0.26020884564615598386e1 * t96874 * t7654 - 0.19756347548806534796e1 * t27011 * t3585 + 0.39512695097613069591e1 * t27011 * t3576 - 0.26020884564615598386e1 * t96883 * t26909 - 0.26020884564615598386e1 * t96888 * t96889 * t13043 * t13149 + 0.26020884564615598386e1 * t96888 * t26907 * t13043 * t13143 + 0.13010442282307799193e1 * t96883 * t26913 - 0.4336814094102599731e0 * t96888 * t7660 * t13043 * t13129 - 0.78062653693846795158e1 * t7651 * t26969 * t26987 * t1294 - 0.13010442282307799193e1 * t96910 * t7662;
    (t96913,)
}
