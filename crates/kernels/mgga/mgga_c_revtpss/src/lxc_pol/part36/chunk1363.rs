//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1363/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1363<F: Float>(t105512: F, t112018: F, t112048: F, t112943: F, t1287: F, t1769: F, t1774: F, t1775: F, t2142: F, t24524: F, t24616: F, t24770: F, t25016: F, t26949: F, t26969: F, t26994: F, t29220: F, t30739: F, t30751: F, t30767: F, t30899: F, t6573: F, t6574: F, t6580: F, t6588: F, t7632: F, t7637: F, t7643: F, t7651: F, t7659: F, t7660: F, t8190: F, t8205: F, t8209: F, t8213: F, t97358: F, t97377: F, t97475: F) -> F {
    let t116565 = F::new(0.52041769129231196772e1) * t26994 * t7637 * t30751 * t1774 - F::new(0.78062653693846795158e1) * t26949 * t7637 * t8190 * t6573 + F::new(0.10408353825846239354e2) * t97358 * t7637 * t2142 * t24616 - F::new(0.13010442282307799193e1) * t8205 * t30899 + F::new(0.39512695097613069591e1) * t29220 * t6580 - F::new(0.13010442282307799193e1) * t112048 * t8213 - F::new(0.4336814094102599731e0) * t7659 * t7660 * t24770 * t1287 - F::new(0.65854491829355115987e0) * t7632 * t25016 - F::new(0.19756347548806534796e1) * t112018 * t1775 - F::new(0.19756347548806534796e1) * t29220 * t6588 + F::new(0.10408353825846239354e2) * t7651 * t97377 * t2142 * t24524 + F::new(0.39512695097613069591e1) * t105512 * t6574 + F::new(0.52041769129231196772e1) * t112943 * t8209 + F::new(0.15612530738769359031e2) * t7643 * t26969 * t30767 * t1774 - F::new(0.15612530738769359031e2) * t97475 * t7637 * t30739 * t1769;
    t116565
}
