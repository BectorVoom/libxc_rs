//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1324/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1324(t2142: f64, t3552: f64, t1204: f64, t1214: f64, t1215: f64, t1248: f64, t12621: f64, t1287: f64, t1295: f64, t13183: f64, t26886: f64, t26895: f64, t26924: f64, t26937: f64, t26945: f64, t26949: f64, t26962: f64, t26969: f64, t26971: f64, t26988: f64, t26994: f64, t27028: f64, t3568: f64, t3738: f64, t7627: f64, t7637: f64, t7643: f64, t7651: f64, t96981: f64, t97377: f64, t97397: f64, t97398: f64, t97402: f64, t97419: f64, t97422: f64) -> f64 {
    let t97425 = t3552 * t2142;
    let t97428 = 0.10408353825846239354e2_f64 * t7651 * t97377 * t2142 * t13183 + 0.19756347548806534796e1_f64 * t1204 * t26886 + 0.52041769129231196772e1_f64 * t26994 * t7637 * t26962 * t1214 - 0.78062653693846795158e1_f64 * t7651 * t26969 * t7627 * t3738 + 0.8673628188205199462e0_f64 * t7643 * t7637 * t2142 * t12621 - 0.26020884564615598386e1_f64 * t97397 * t96981 * t97398 - 0.39512695097613069591e1_f64 * t97402 * t1215 + 0.52041769129231196772e1_f64 * t26937 * t26945 - 0.78062653693846795158e1_f64 * t26937 * t26971 + 0.26020884564615598386e1_f64 * t26937 * t26988 + 0.52041769129231196772e1_f64 * t26895 * t27028 * t1248 * t1287 - 0.78062653693846795158e1_f64 * t26949 * t7637 * t7627 * t3568 - 0.19756347548806534796e1_f64 * t97419 * t1215 + 0.52041769129231196772e1_f64 * t97422 * t26924 - 0.19756347548806534796e1_f64 * t97425 * t1295;
    t97428
}
