//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1161/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1161<F: Float>(t45551: F, t473: F, t1243: F, t2149: F, t37885: F, t1294: F, t21471: F, t3555: F, t7627: F, t1209: F, t26884: F, t26921: F, t7648: F, t2142: F, t3552: F, t1204: F, t1214: F, t1215: F, t1248: F, t12621: F, t1287: F, t1295: F, t13183: F, t26886: F, t26895: F, t26924: F, t26937: F, t26945: F, t26949: F, t26962: F, t26969: F, t26971: F, t26988: F, t26994: F, t27028: F, t3568: F, t3738: F, t7637: F, t7643: F, t7651: F, t96981: F) -> (F,) {
    let t97377 = t45551 * t473;
    let t97397 = t2149 * t37885 * t1243;
    let t97398 = t21471 * t1294;
    let t97402 = t3555 * t7627;
    let t97419 = t1209 * t26884;
    let t97422 = t7648 * t26921;
    let t97425 = t3552 * t2142;
    let t97428 = 0.10408353825846239354e2 * t7651 * t97377 * t2142 * t13183 + 0.19756347548806534796e1 * t1204 * t26886 + 0.52041769129231196772e1 * t26994 * t7637 * t26962 * t1214 - 0.78062653693846795158e1 * t7651 * t26969 * t7627 * t3738 + 0.8673628188205199462e0 * t7643 * t7637 * t2142 * t12621 - 0.26020884564615598386e1 * t97397 * t96981 * t97398 - 0.39512695097613069591e1 * t97402 * t1215 + 0.52041769129231196772e1 * t26937 * t26945 - 0.78062653693846795158e1 * t26937 * t26971 + 0.26020884564615598386e1 * t26937 * t26988 + 0.52041769129231196772e1 * t26895 * t27028 * t1248 * t1287 - 0.78062653693846795158e1 * t26949 * t7637 * t7627 * t3568 - 0.19756347548806534796e1 * t97419 * t1215 + 0.52041769129231196772e1 * t97422 * t26924 - 0.19756347548806534796e1 * t97425 * t1295;
    (t97428,)
}
