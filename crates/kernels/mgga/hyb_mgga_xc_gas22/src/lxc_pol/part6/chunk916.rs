//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 916/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk916<F: Float>(t7: F, t132: F, t8473: F, t8530: F, t8553: F, t8583: F, t1793: F, t5891: F, t1370: F, t6666: F, t2291: F, t6669: F, t2311: F, t3418: F, t3444: F, t3443: F, t6737: F, t1347: F, t2228: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t8586 = piecewise3(t134, 0.0, t8473 + t8530 + t8553 + t8583);
    let t8587 = 2.0 * t1793;
    let t8588 = 6.0 * t5891;
    let t8589 = t8587 - t8588;
    let t8590 = piecewise3(t8, 0.0, t8589);
    let t8599 = t6666 * t1370;
    let t8600 = t6669 * t2291;
    let t8601 = t8599 * t8600;
    let t8604 = t2311 * t3418;
    let t8605 = t8604 * t3444;
    let t8608 = t3443 * t6737;
    let t8611 = t1347 * t2228;
    (t8586, t8587, t8588, t8589, t8590, t8600, t8601, t8605, t8608, t8611)
}
