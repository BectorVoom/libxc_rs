//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 542/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk542<F: Float>(t10309: F, t10313: F, t10317: F, t10321: F, t10323: F, t10326: F, t10329: F, t10331: F, t9266: F, t9270: F, t9276: F, t9281: F, t9289: F, t9296: F, t9307: F) -> F {
    let t10332 = -t9266 + t9270 - t9276 - t10309 - t10313 - t10317 - t10321 + t10323 - t9281 + t9289 + t9296 - t9307 - t10326 + t10329 + t10331;
    t10332
}
