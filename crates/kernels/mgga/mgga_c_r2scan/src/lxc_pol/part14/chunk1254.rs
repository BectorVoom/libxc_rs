//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1254/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1254<F: Float>(t37390: F, t39290: F, t39309: F, t41223: F, t41225: F, t41227: F, t41230: F, t41233: F, t41236: F, t41240: F, t41243: F, t41247: F, t41251: F, t41254: F, t41256: F) -> F {
    let t42175 = -t41223 + F::new(0.3842256877732895568e-2) * t37390 + t41225 + t41227 + F::new(0.60975299583150056624e-3) * t39290 - t41230 - t41233 - t41236 + F::new(0.30487649791575028312e-3) * t39309 - t41240 + t41243 + t41247 - t41251 - t41254 - t41256;
    t42175
}
