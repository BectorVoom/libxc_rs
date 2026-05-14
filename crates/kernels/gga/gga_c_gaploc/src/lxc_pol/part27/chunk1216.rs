//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1216/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1216<F: Float>(t12001: F, t12044: F, t12069: F, t1441: F, t1562: F, t34288: F, t34291: F, t34294: F, t34297: F, t34299: F, t34301: F, t34303: F, t34305: F, t34307: F, t34309: F, t34311: F, t34314: F, t34318: F, t4614: F, t574: F, t590: F) -> (F,) {
    let t38522 = -0.18404604457881959845e2 * t1562 * t4614 * t12069 - 0.12269736305254639896e2 * t574 * t4614 * t12044 + 0.1022478025437886658e1 * t1441 * t12001 * t590 - t34288 + t34291 - t34294 + t34297 + t34299 + t34301 + t34303 + t34305 - t34307 - t34309 - t34311 + t34314 - t34318;
    (t38522,)
}
