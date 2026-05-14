//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 796/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk796<F: Float>(t1732: F, t2758: F, t5754: F, t5761: F, t5766: F, t5770: F, t5774: F, t5901: F, t5907: F, t5908: F, t5910: F, t5912: F, t5919: F, t625: F, t898: F, t1768: F) -> (F, F, F) {
    let t7817 = t2758 * t1732;
    let t7822 = -0.10005107252466666666e-2 * t7817 + t5901 - t5754 + t5907 + 0.65061487801810439052e-1 * t5908 + 0.1301229756036208781e0 * t5910 + 0.38527786510141256862e1 * t5912 + t5761 + t5766 + t5770 - t5774 + t5919;
    let t7824 = t898 * t625;
    let t7825 = t7824 * t1768;
    (t7822, t7824, t7825)
}
