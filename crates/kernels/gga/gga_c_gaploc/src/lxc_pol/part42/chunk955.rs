//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 955/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk955<F: Float>(t1063: F, t11977: F, t3701: F, t44328: F, t44334: F, t44336: F, t44350: F, t44355: F, t44358: F, t44363: F, t44367: F, t44371: F, t44375: F, t44377: F, t44390: F, t44394: F, t44403: F, t44409: F, t44410: F, t7974: F, t8207: F) -> F {
    let t49859 = t44328 + t44334 + t44336 - t44350 - t44355 - t44358 + t44363 - t44367 + t44371 - t44375 + t44377 + F::new(0.1707300398140568976e0) * t1063 * t11977 * t8207 - F::new(0.56910013271352299198e-1) * t1063 * t3701 * t7974 + t44390 + t44394 - t44403 + t44409 - t44410;
    t49859
}
