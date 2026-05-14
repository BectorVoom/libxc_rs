//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1061/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1061<F: Float>(t3275: F, t3472: F, t39286: F, t41196: F, t41199: F, t41201: F, t41205: F, t41208: F, t41211: F, t41213: F, t41216: F, t41219: F, t41221: F, t41223: F, t41225: F, t41227: F, t41230: F, t41233: F) -> (F, F) {
    let t41236 = 5.0 / 16.0 * t3275 * t3472 * t39286;
    let t41237 = t41196 + t41199 + t41201 + t41205 + t41208 - t41211 + t41213 - t41216 - t41219 + t41221 + t41223 - t41225 - t41227 + t41230 + t41233 + t41236;
    (t41236, t41237)
}
