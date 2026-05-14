//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1238/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1238<F: Float>(t5720: F, t898: F, t1422: F, t2794: F, t1721: F, t2483: F, t22491: F, t1734: F, t7829: F, t1726: F, t1727: F, t2782: F, t22232: F, t7755: F, t410: F, t7794: F) -> (F, F, F, F, F, F, F, F) {
    let t26814 = t898 * t5720;
    let t26819 = t1422 * t2794;
    let t26820 = 96.0 * t26819;
    let t26823 = t2483 * t1721;
    let t26824 = 0.19518446340543131715e0 * t26823;
    let t26831 = 480.0 * t22491;
    let t26835 = t7829 * t1734;
    let t26838 = t1726 * t2782 * t1727;
    let t26839 = 0.5143752e0 * t26838;
    let t26849 = t7755 * t22232;
    let t26860 = t410 * t7794;
    (t26814, t26820, t26824, t26831, t26835, t26839, t26849, t26860)
}
