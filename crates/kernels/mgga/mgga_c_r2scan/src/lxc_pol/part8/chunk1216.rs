//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1216/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1216<F: Float>(t13866: F, t565: F, t1592: F, t2654: F, t551: F, t6343: F, t1610: F, t2207: F, t8270: F, t2837: F, t6263: F, t783: F, t19875: F, t25214: F, t545: F, t2599: F, t3433: F) -> (F, F, F, F, F, F) {
    let t25766 = t565 * t13866;
    let t25779 = t1592 * t551 * t6343 * t2654;
    let t25780 = 0.38140175656238781678e1 * t25779;
    let t25797 = t2207 * t1610 * t8270;
    let t25798 = 0.6112917064160653851e0 * t25797;
    let t25800 = t783 * t2837 * t6263;
    let t25804 = t545 * t19875 * t25214;
    let t25826 = t3433 * t2599;
    (t25766, t25780, t25798, t25800, t25804, t25826)
}
