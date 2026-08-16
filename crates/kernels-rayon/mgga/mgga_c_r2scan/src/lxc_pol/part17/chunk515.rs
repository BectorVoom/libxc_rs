//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 515/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk515(t170: f64, t2768: f64, t1730: f64, t1747: f64, t1750: f64, t1752: f64, t1756: f64, t1761: f64, t1766: f64, t1770: f64, t1772: f64, t1788: f64, t1794: f64, t1796: f64, t2755: f64, t2759: f64, t2761: f64, t2763: f64, t2765: f64, t41: f64, t596: f64) -> (f64, f64) {
    let t2769 = t2768 * t170;
    let t2772 = t1730 + t1747 + t1750 - 0.54217906501508699211e-2_f64 * t1752 - t1756 - t1761 - t41 * t2755 + t1766 - t1770 - 0.20010214504933333333e-2_f64 * t1772 - 0.20010214504933333333e-2_f64 * t2759 - 4.0_f64 * t2761 + 4.0_f64 * t2763 + t1788 + 0.65061487801810439052e-1_f64 * t2765 - t1794 + 4.0_f64 * t1796 - 0.675260332e-1_f64 * t596 * t2769;
    (t2769, t2772)
}
