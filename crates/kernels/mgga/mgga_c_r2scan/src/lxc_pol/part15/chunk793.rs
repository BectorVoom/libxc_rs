//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 793/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk793<F: Float>(t1678: F, t955: F, t159: F, t1686: F, t170: F, t7028: F, t5474: F, t5479: F, t5585: F, t5601: F, t5605: F, t5846: F, t5847: F, t5853: F, t5855: F, t5864: F) -> (F,) {
    let t7783 = t955 * t1678;
    let t7784 = t159 * t7783;
    let t7785 = t7784 * t1686;
    let t7788 = t7028 * t170;
    let t7792 = 0.42340699333333333333e-3 * t7785 + t5474 - t5479 - t5846 + 24.0 * t5847 + t5853 - t5585 + 0.285764e-1 * t159 * t7788 - 0.1143056e0 * t5855 - t5864 - t5601 - t5605;
    (t7792,)
}
