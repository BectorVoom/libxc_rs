//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1416/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1416<F: Float>(t26770: F, t22228: F, t7755: F, t595: F, t637: F, t7098: F, t7101: F, t1732: F, t7808: F, t7104: F, t7811: F, t5203: F, t7824: F, t216: F, t21914: F, t21923: F, t21927: F, t21930: F, t22424: F, t23893: F) -> (F,) {
    let t26771 = 3.0 * t26770;
    let t26773 = t7755 * t22228;
    let t26778 = t595 * t7098 * t637;
    let t26781 = t595 * t7101 * t637;
    let t26783 = t7808 * t1732;
    let t26786 = t595 * t7104 * t637;
    let t26788 = t7811 * t1732;
    let t26790 = t7824 * t5203;
    let t26792 = -t21914 - t26771 - t21923 + 72.0 * t22424 + t21927 - t21930 + 0.1200612870296e-1 * t26773 - 0.21973736767207854065e-2 * t23893 * t216 - 0.60030643514799999999e-2 * t26778 - 0.1200612870296e-1 * t26781 - 0.30015321757399999999e-2 * t26783 - 0.60030643514799999999e-2 * t26786 - 0.30015321757399999999e-2 * t26788 - 0.48159733137676571079e0 * t26790;
    (t26792,)
}
