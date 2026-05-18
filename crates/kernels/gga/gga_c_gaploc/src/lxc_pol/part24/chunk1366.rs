//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1366/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1366<F: Float>(t34310: F, t34239: F, t4391: F, t6964: F, t34276: F, t34279: F, t34282: F, t34285: F, t34288: F, t34291: F, t34294: F, t34297: F, t34299: F, t34301: F, t34303: F, t34305: F, t34307: F, t34309: F) -> F {
    let t34311 = F::new(0.59584149919750711116e-1) * t34310;
    let t34314 = F::new(0.85801175884441024006e1) * t4391 * t6964 * t34239;
    let t34315 = t34276 - t34279 - t34282 + t34285 - t34288 + t34291 - t34294 + t34297 + t34299 + t34301 + t34303 + t34305 - t34307 - t34309 - t34311 + t34314;
    t34315
}
