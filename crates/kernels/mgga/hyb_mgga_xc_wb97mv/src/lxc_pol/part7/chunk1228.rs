//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1228/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1228<F: Float>(t7: F, t29051: F, t29095: F, t29132: F, t29663: F, t29728: F, t29805: F, t29850: F, t29873: F, t1234: F, t8861: F, t8619: F, t10846: F, t1283: F, t1312: F, t2015: F, t214: F, t25581: F, t25583: F, t25586: F, t25588: F, t25590: F, t25593: F, t25595: F, t25597: F, t25600: F, t25603: F, t25614: F, t25627: F, t3003: F, t4069: F, t676: F, t683: F, t685: F, t8854: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t29877 = piecewise3(t9, 0.0, t29051 + t29095 + t29132 + t29663 + t29728 + t29805 + t29850 + t29873);
    let t29897 = t1234 * t8861;
    let t29899 = t1234 * t8619;
    let t29907 = t25581 / 48.0 + t25583 / 24.0 - 5.0 / 144.0 * t25586 - 5.0 / 144.0 * t25588 + t25590 / 8.0 - t25593 / 32.0 - t25595 / 32.0 - t25597 / 16.0 + t25600 / 24.0 - 5.0 / 144.0 * t25603 - 3.0 / 64.0 * t2015 * t4069 - 3.0 / 32.0 * t676 * t10846 + t683 * t3003 * t685 * t1283 * t214 / 16.0 + t29897 / 48.0 + t29899 / 48.0 + t683 * t3003 * t8854 * t1312 / 16.0 - t25614 / 32.0 - t25627 / 32.0;
    (t29877, t29907)
}
