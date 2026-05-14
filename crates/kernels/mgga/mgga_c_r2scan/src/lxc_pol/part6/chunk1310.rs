//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1310/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1310<F: Float>(t24755: F, t1616: F, t783: F, t8279: F, t1543: F, t2841: F, t22980: F, t1604: F, t20110: F, t20113: F, t20747: F, t24409: F, t24733: F, t24735: F, t24742: F, t24745: F, t24748: F, t24752: F, t506: F, t527: F, t529: F, t940: F) -> (F, F) {
    let t24756 = 0.2037639021386884617e0 * t24755;
    let t24758 = t783 * t8279 * t1616;
    let t24759 = 0.2037639021386884617e0 * t24758;
    let t24762 = t2841 * t1543;
    let t24763 = t22980 * t24762;
    let t24764 = t1604 * t24763;
    let t24766 = -t24733 - t24735 - 0.54878743191129263322e-1 * t527 * t529 * t506 * t24409 - 0.43341108700271342816e-1 * t20747 * t940 + 0.18496169001454677638e1 * t24742 + 0.29272321618148349056e-1 * t24745 - 0.14636160809074174528e-1 * t24748 + 0.17465477326173296717e-1 * t24752 - t24756 - t24759 - 0.52396431978519890151e-1 * t20110 + 0.20958572791407956061e0 * t20113 + 0.19756347548806534797e0 * t24764;
    (t24763, t24766)
}
