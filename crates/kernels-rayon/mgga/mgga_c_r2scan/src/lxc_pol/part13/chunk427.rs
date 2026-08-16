//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 427/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk427(t170: f64, t1783: f64, t410: f64, t726: f64, t1376: f64, t229: f64, t159: f64, t1709: f64, t1710: f64, t1723: f64, t1730: f64, t1747: f64, t1750: f64, t1752: f64, t1756: f64, t1761: f64, t1766: f64, t1770: f64, t1772: f64, t41: f64) -> (f64, f64, f64, f64) {
    let t1784 = t1783 * t170;
    let t1788 = 8.0_f64 * t410 * t726;
    let t1789 = t1376 * t229;
    let t1791 = t1709 + 0.1301229756036208781e0_f64 * t1710 + t1723 + t1730 + t1747 + t1750 - 0.10843581300301739842e-1_f64 * t1752 - t1756 - t1761 + t1766 - t1770 - 0.40020429009866666666e-2_f64 * t1772 + 0.285764e-1_f64 * t159 * t1784 + t1788 - t41 * t1789;
    (t1784, t1788, t1789, t1791)
}
