//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 660/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk660(t11688: f64, t549: f64, t1036: f64, t10854: f64, t10858: f64, t10863: f64, t10868: f64, t11743: f64, t11749: f64, t11752: f64, t11757: f64, t11760: f64, t11765: f64, t1998: f64, t2009: f64, t2033: f64, t6060: f64, t780: f64, t8634: f64) -> f64 {
    let t11772 = t549 * t11688;
    let t11775 = -0.35750489951850426669e0_f64 * t11743 * t2009 + 0.71500979903700853338e0_f64 * t1036 * t8634 - 0.23005755572352449806e1_f64 * t1998 * t11749 - 0.21450293971110256001e1_f64 * t6060 * t11752 + 0.35750489951850426669e0_f64 * t780 * t11757 - 0.35750489951850426669e0_f64 * t11760 * t2009 + 0.35750489951850426669e0_f64 * t780 * t11765 - 0.59584149919750711116e-1_f64 * t10854 - 0.59584149919750711116e-1_f64 * t10858 + 0.59584149919750711116e-1_f64 * t10863 - 0.17875244975925213335e0_f64 * t10868 + 0.39722766613167140743e-1_f64 * t2033 * t11772;
    t11775
}
