//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 626/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk626<F: Float>(t11688: F, t549: F, t1036: F, t10854: F, t10858: F, t10863: F, t10868: F, t11743: F, t11749: F, t11752: F, t11757: F, t11760: F, t11765: F, t1998: F, t2009: F, t2033: F, t6060: F, t780: F, t8634: F) -> F {
    let t11772 = t549 * t11688;
    let t11775 = -F::cast_from(0.35750489951850426669e0_f64) * t11743 * t2009 + F::cast_from(0.71500979903700853338e0_f64) * t1036 * t8634 - F::cast_from(0.23005755572352449806e1_f64) * t1998 * t11749 - F::cast_from(0.21450293971110256001e1_f64) * t6060 * t11752 + F::cast_from(0.35750489951850426669e0_f64) * t780 * t11757 - F::cast_from(0.35750489951850426669e0_f64) * t11760 * t2009 + F::cast_from(0.35750489951850426669e0_f64) * t780 * t11765 - F::cast_from(0.59584149919750711116e-1_f64) * t10854 - F::cast_from(0.59584149919750711116e-1_f64) * t10858 + F::cast_from(0.59584149919750711116e-1_f64) * t10863 - F::cast_from(0.17875244975925213335e0_f64) * t10868 + F::cast_from(0.39722766613167140743e-1_f64) * t2033 * t11772;
    t11775
}
