//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1380/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1380(t12001: f64, t12044: f64, t12069: f64, t1441: f64, t1562: f64, t34288: f64, t34291: f64, t34294: f64, t34297: f64, t34299: f64, t34301: f64, t34303: f64, t34305: f64, t34307: f64, t34309: f64, t34311: f64, t34314: f64, t34318: f64, t4614: f64, t574: f64, t590: f64) -> f64 {
    let t38522 = -0.18404604457881959845e2_f64 * t1562 * t4614 * t12069 - 0.12269736305254639896e2_f64 * t574 * t4614 * t12044 + 0.1022478025437886658e1_f64 * t1441 * t12001 * t590 - t34288 + t34291 - t34294 + t34297 + t34299 + t34301 + t34303 + t34305 - t34307 - t34309 - t34311 + t34314 - t34318;
    t38522
}
