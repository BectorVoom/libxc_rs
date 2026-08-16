//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1363/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1363(t105512: f64, t112018: f64, t112048: f64, t112943: f64, t1287: f64, t1769: f64, t1774: f64, t1775: f64, t2142: f64, t24524: f64, t24616: f64, t24770: f64, t25016: f64, t26949: f64, t26969: f64, t26994: f64, t29220: f64, t30739: f64, t30751: f64, t30767: f64, t30899: f64, t6573: f64, t6574: f64, t6580: f64, t6588: f64, t7632: f64, t7637: f64, t7643: f64, t7651: f64, t7659: f64, t7660: f64, t8190: f64, t8205: f64, t8209: f64, t8213: f64, t97358: f64, t97377: f64, t97475: f64) -> f64 {
    let t116565 = 0.52041769129231196772e1_f64 * t26994 * t7637 * t30751 * t1774 - 0.78062653693846795158e1_f64 * t26949 * t7637 * t8190 * t6573 + 0.10408353825846239354e2_f64 * t97358 * t7637 * t2142 * t24616 - 0.13010442282307799193e1_f64 * t8205 * t30899 + 0.39512695097613069591e1_f64 * t29220 * t6580 - 0.13010442282307799193e1_f64 * t112048 * t8213 - 0.4336814094102599731e0_f64 * t7659 * t7660 * t24770 * t1287 - 0.65854491829355115987e0_f64 * t7632 * t25016 - 0.19756347548806534796e1_f64 * t112018 * t1775 - 0.19756347548806534796e1_f64 * t29220 * t6588 + 0.10408353825846239354e2_f64 * t7651 * t97377 * t2142 * t24524 + 0.39512695097613069591e1_f64 * t105512 * t6574 + 0.52041769129231196772e1_f64 * t112943 * t8209 + 0.15612530738769359031e2_f64 * t7643 * t26969 * t30767 * t1774 - 0.15612530738769359031e2_f64 * t97475 * t7637 * t30739 * t1769;
    t116565
}
