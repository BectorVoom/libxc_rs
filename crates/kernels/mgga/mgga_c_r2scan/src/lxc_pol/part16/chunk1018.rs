//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1018/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1018<F: Float>(t12580: F, t354: F, t1044: F, t3685: F, t1108: F, t2995: F, t1100: F, t3250: F, t11036: F, t2928: F, t2938: F, t3358: F) -> (F, F, F, F, F, F) {
    let t12581 = t354 * t12580;
    let t12582 = t3685 * t1044;
    let t12583 = F::cast_from(2.0_f64) * t12582;
    let t12584 = t2995 * t1108;
    let t12585 = t1100 * t3250;
    let t12587 = t11036 * t2928;
    let t12589 = t3358 * t2938;
    (t12581, t12583, t12584, t12585, t12587, t12589)
}
