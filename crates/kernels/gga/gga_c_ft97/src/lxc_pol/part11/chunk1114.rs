//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1114/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1114<F: Float>(t10254: F, t1882: F, t43348: F, t43353: F, t43357: F, t43361: F, t43363: F, t43365: F, t43369: F, t43373: F, t43376: F, t43379: F, t43384: F, t43388: F, t43390: F, t43392: F) -> (F, F) {
    let t43394 = t1882 * t10254;
    let t43396 = -F::new(8.0) / F::new(27.0) * t43348 - F::new(8.0) / F::new(9.0) * t43353 - F::new(4.0) / F::new(3.0) * t43357 - F::new(16.0) / F::new(9.0) * t43361 - F::new(8.0) / F::new(27.0) * t43363 - F::new(8.0) / F::new(9.0) * t43365 - F::new(4.0) / F::new(3.0) * t43369 + F::new(8.0) / F::new(3.0) * t43373 - F::new(4.0) * t43376 + F::new(8.0) / F::new(3.0) * t43379 + F::new(8.0) / F::new(3.0) * t43384 + F::new(8.0) / F::new(3.0) * t43388 + F::new(16.0) / F::new(27.0) * t43390 + F::new(8.0) / F::new(9.0) * t43392 + F::new(8.0) / F::new(9.0) * t43394;
    (t43394, t43396)
}
