//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 502/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk502<F: Float>(t170: F, t2768: F, t1730: F, t1747: F, t1750: F, t1752: F, t1756: F, t1761: F, t1766: F, t1770: F, t1772: F, t1788: F, t1794: F, t1796: F, t2755: F, t2759: F, t2761: F, t2763: F, t2765: F, t41: F, t596: F) -> (F, F) {
    let t2769 = t2768 * t170;
    let t2772 = t1730 + t1747 + t1750 - 0.54217906501508699211e-2 * t1752 - t1756 - t1761 - t41 * t2755 + t1766 - t1770 - 0.20010214504933333333e-2 * t1772 - 0.20010214504933333333e-2 * t2759 - 4.0 * t2761 + 4.0 * t2763 + t1788 + 0.65061487801810439052e-1 * t2765 - t1794 + 4.0 * t1796 - 0.675260332e-1 * t596 * t2769;
    (t2769, t2772)
}
