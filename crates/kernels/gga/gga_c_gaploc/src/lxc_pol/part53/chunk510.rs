//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 510/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk510<F: Float>(t7030: F, t9305: F, t1424: F, t4386: F, t4391: F, t9266: F, t9270: F, t9276: F, t9281: F, t9282: F, t9289: F, t9291: F, t9296: F, t9298: F, t9302: F) -> (F, F) {
    let t9307 = F::new(0.29792074959875355558e-1) * t9305 * t7030;
    let t9308 = -t9266 + t9270 - t9276 - t9281 - F::new(0.79445533226334281487e-1) * t4391 * t9282 + t9289 + F::new(0.11916829983950142223e0) * t9291 * t4386 + t9296 - F::new(0.39722766613167140743e-1) * t9298 * t1424 - F::new(0.39722766613167140743e-1) * t9302 * t1424 - t9307;
    (t9307, t9308)
}
