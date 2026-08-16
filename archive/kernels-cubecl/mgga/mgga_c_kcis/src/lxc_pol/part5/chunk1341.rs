//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1341/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1341<F: Float>(t518: F, t6957: F, t1419: F, t5457: F, t5458: F, t5481: F, t1098: F, t7242: F, t3814: F, t531: F, t21641: F, t16373: F, t21625: F) -> (F, F, F, F, F, F, F) {
    let t22164 = t518 * t6957;
    let t22165 = t22164 * t1419;
    let t22166 = t5457 * t22165;
    let t22169 = t5458 * t5481;
    let t22170 = t5457 * t22169;
    let t22175 = t1098 * t7242;
    let t22177 = t3814 * t531;
    let t22178 = t22177 * t21641;
    let t22181 = t16373 * t21625;
    (t22165, t22166, t22169, t22170, t22175, t22178, t22181)
}
