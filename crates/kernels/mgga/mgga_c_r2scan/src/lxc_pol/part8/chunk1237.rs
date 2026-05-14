//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1237/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1237<F: Float>(t1422: F, t2755: F, t22406: F, t2035: F, t2483: F, t41: F, t22228: F, t7755: F, t1732: F, t7808: F, t7811: F, t5203: F, t7824: F, t22336: F, t22340: F, t22452: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26758 = 96.0 * t1422 * t2755;
    let t26765 = 144.0 * t22406;
    let t26770 = t41 * t2483 * t2035;
    let t26771 = 3.0 * t26770;
    let t26773 = t7755 * t22228;
    let t26783 = t7808 * t1732;
    let t26788 = t7811 * t1732;
    let t26790 = t7824 * t5203;
    let t26803 = 31680.0 * t22336;
    let t26804 = 52416.0 * t22340;
    let t26813 = 12.0 * t22452;
    (t26758, t26765, t26771, t26773, t26783, t26788, t26790, t26803, t26804, t26813)
}
