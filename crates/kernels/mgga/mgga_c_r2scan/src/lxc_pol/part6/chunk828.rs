//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 828/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk828<F: Float>(t170: F, t5879: F, t2036: F, t406: F, t1419: F, t726: F, t159: F, t5585: F, t5601: F, t5605: F, t5609: F, t5612: F, t5614: F, t5669: F, t5678: F, t5682: F, t5689: F, t5853: F, t5855: F, t5864: F, t5868: F) -> (F, F, F, F) {
    let t5880 = t5879 * t170;
    let t5883 = t406 * t2036;
    let t5884 = 12.0 * t5883;
    let t5885 = t1419 * t726;
    let t5886 = 36.0 * t5885;
    let t5887 = t5853 - t5585 - 0.1714584e0 * t5855 - t5864 - t5601 - t5605 + t5609 + t5612 - t5614 + t5868 + 0.285764e-1 * t159 * t5880 - t5884 - t5669 - t5678 - t5682 - t5689 - t5886;
    (t5880, t5883, t5885, t5887)
}
