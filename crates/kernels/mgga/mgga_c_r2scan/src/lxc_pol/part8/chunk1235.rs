//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1235/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1235<F: Float>(t22360: F, t5961: F, t7824: F, t5200: F, t1734: F, t7808: F, t7811: F, t2758: F, t5416: F, t1732: F, t7829: F, t22166: F, t7693: F, t1823: F, t2747: F, t5261: F, t963: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26700 = 240.0 * t22360;
    let t26704 = t7824 * t5961;
    let t26706 = t7824 * t5200;
    let t26708 = t7808 * t1734;
    let t26710 = t7811 * t1734;
    let t26712 = t2758 * t5416;
    let t26720 = t7829 * t1732;
    let t26721 = 0.300153217574e-2 * t26720;
    let t26724 = t7693 * t22166;
    let t26725 = 0.12154685976e1 * t26724;
    let t26728 = t2747 * t1823;
    let t26729 = 0.30762056574649219973e4 * t26728;
    let t26730 = t963 * t5261;
    (t26700, t26704, t26706, t26708, t26710, t26712, t26721, t26725, t26729, t26730)
}
