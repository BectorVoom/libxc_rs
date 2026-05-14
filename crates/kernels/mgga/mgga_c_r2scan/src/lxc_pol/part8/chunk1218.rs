//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1218/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1218<F: Float>(t1575: F, t784: F, t546: F, t565: F, t2719: F, t549: F, t551: F, t6343: F, t11747: F, t545: F, t19890: F, t6085: F, t7922: F, t6535: F, t8089: F, t6407: F, t8090: F) -> (F, F, F, F, F, F, F) {
    let t25966 = t1575 * t784;
    let t25967 = t546 * t25966;
    let t25972 = t565 * t25966;
    let t25978 = t549 * t551 * t6343 * t2719;
    let t25979 = 0.12713391885412927226e1 * t25978;
    let t25983 = t545 * t11747;
    let t26007 = t6085 * t19890 * t7922;
    let t26008 = 0.2037639021386884617e0 * t26007;
    let t26018 = t6535 * t19890 * t8089;
    let t26020 = t6407 * t8090;
    (t25967, t25972, t25979, t25983, t26008, t26018, t26020)
}
