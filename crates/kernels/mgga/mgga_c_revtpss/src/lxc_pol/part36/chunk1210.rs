//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1210/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1210<F: Float>(t1828: F, t5457: F, t104529: F, t105090: F, t105509: F, t111815: F, t112075: F, t112121: F, t112686: F, t112721: F, t1287: F, t1769: F, t1774: F, t1794: F, t1829: F, t21471: F, t2148: F, t2151: F, t2152: F, t24864: F, t25019: F, t26906: F, t26922: F, t26994: F, t29122: F, t29227: F, t29304: F, t30747: F, t30860: F, t30867: F, t3769: F, t3783: F, t6580: F, t6622: F, t6628: F, t6745: F, t7637: F, t7659: F, t96861: F, t97066: F, t97318: F) -> (F,) {
    let t116390 = t5457 * t1828;
    let t116430 = -0.4336814094102599731e0 * t2148 * t24864 * t2152 - 0.20816707651692478709e2 * t97066 * t2151 * t112721 * t1774 + 0.52041769129231196772e1 * t26922 * t112121 * t116390 + 0.26020884564615598386e1 * t97318 * t111815 * t21471 * t1769 - 0.19756347548806534796e1 * t29227 * t6745 - 0.39512695097613069591e1 * t112075 * t1829 - 0.39512695097613069591e1 * t96861 * t25019 + 0.39512695097613069591e1 * t29304 * t6580 + 0.10408353825846239354e2 * t26994 * t7637 * t30747 * t1769 - 0.13010442282307799193e1 * t7659 * t112686 * t1794 * t1287 - 0.13010442282307799193e1 * t7659 * t29122 * t6622 * t1287 - 0.26020884564615598386e1 * t26906 * t105090 * t6628 * t3769 + 0.13010442282307799193e1 * t26906 * t29122 * t6628 * t3783 + 0.13010442282307799193e1 * t104529 * t30860 + 0.10408353825846239354e2 * t105509 * t30867;
    (t116430,)
}
