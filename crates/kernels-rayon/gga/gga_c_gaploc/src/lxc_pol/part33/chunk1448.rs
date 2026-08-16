//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1448/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1448(t12306: f64, t1935: f64, t29304: f64, t29310: f64, t32272: f64, t32275: f64, t32277: f64, t32281: f64, t32285: f64, t32289: f64, t32329: f64, t32332: f64, t32334: f64, t32337: f64, t3723: f64, t681: f64) -> f64 {
    let t39383 = -t32272 + t32275 - t32277 - t32281 - t32285 - t32289 + t29304 + 0.76905262301422242837e-2_f64 * t1935 * t3723 + 0.15381052460284448567e-1_f64 * t681 * t12306 + t29310 - t32329 - t32332 + t32334 + t32337;
    t39383
}
