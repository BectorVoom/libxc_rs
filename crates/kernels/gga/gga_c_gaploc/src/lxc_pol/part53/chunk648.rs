//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 648/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk648<F: Float>(t10395: F, t10398: F, t10401: F, t10404: F, t10411: F, t10412: F, t10414: F, t10415: F, t10416: F, t10423: F, t10426: F, t10428: F, t10433: F, t10437: F, t10441: F, t10443: F) -> F {
    let t12086 = -t10395 - t10398 + t10401 - t10404 - t10411 + t10412 + t10414 - t10415 + t10416 - t10423 + t10426 + t10428 + t10433 - t10437 + t10441 - t10443;
    t12086
}
