//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1010/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1010<F: Float>(t2035: F, t898: F, t41: F, t5883: F, t5885: F, t1745: F, t963: F, t5609: F, t5612: F, t5614: F, t5669: F, t5678: F, t5682: F, t5689: F, t5868: F, t2747: F, t745: F) -> (F, F, F) {
    let t7794 = t898 * t2035;
    let t7795 = t41 * t7794;
    let t7796 = 4.0 * t5883;
    let t7797 = 12.0 * t5885;
    let t7798 = t963 * t1745;
    let t7800 = t5609 + t5612 - t5614 - t7795 + t5868 - t7796 - t5669 - t5678 - t5682 - t5689 + t7797 + 0.5848223622634646207e0 * t7798;
    let t7802 = 0.11696447245269292414e1 * t2747 * t745;
    (t7794, t7800, t7802)
}
