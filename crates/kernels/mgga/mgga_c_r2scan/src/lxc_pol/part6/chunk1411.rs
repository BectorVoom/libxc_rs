//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1411/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1411<F: Float>(t22360: F, t584: F, t591: F, t7788: F, t5961: F, t7824: F, t5200: F, t1734: F, t7808: F, t7811: F, t2758: F, t5416: F, t595: F, t637: F, t7133: F, t7136: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26700 = 240.0 * t22360;
    let t26702 = t584 * t7788 * t591;
    let t26704 = t7824 * t5961;
    let t26706 = t7824 * t5200;
    let t26708 = t7808 * t1734;
    let t26710 = t7811 * t1734;
    let t26712 = t2758 * t5416;
    let t26715 = t595 * t7133 * t637;
    let t26718 = t595 * t7136 * t637;
    (t26700, t26702, t26704, t26706, t26708, t26710, t26712, t26715, t26718)
}
