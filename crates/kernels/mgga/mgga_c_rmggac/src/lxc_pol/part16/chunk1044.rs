//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1044/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1044<F: Float>(t34847: F, t9990: F, t1528: F, t236: F, t615: F, t7230: F, t7231: F, t4044: F, t46055: F, t5058: F, t8639: F, t8642: F) -> (F, F, F, F) {
    let t47767 = t34847 * t9990;
    let t47772 = t7230 * t7231 * t236 * t1528 * t615;
    let t47774 = t4044 * t46055;
    let t47785 = t5058 * t8639 * t8642;
    (t47767, t47772, t47774, t47785)
}
