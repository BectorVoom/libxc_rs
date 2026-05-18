//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 998/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk998<F: Float>(t10309: F, t10313: F, t10317: F, t10321: F, t10323: F, t10326: F, t10329: F, t12038: F, t1429: F, t9265: F, t9270: F, t9276: F, t9280: F, t9289: F, t9296: F, t9307: F) -> F {
    let t12043 = F::new(0.39722766613167140743e-1) * t1429 * t12038 - F::new(0.76685851907841499354e0) * t9265 + t9270 - t9276 - t10309 - t10313 - t10317 - t10321 + t10323 - F::new(0.38342925953920749677e0) * t9280 + t9289 + t9296 - t9307 - t10326 + t10329;
    t12043
}
