//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2351/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2351(t13242: f64, t1510: f64, t16662: f64, t16891: f64, t16912: f64, t20885: f64, t20887: f64, t20891: f64, t232: f64, t2643: f64, t2645: f64, t2647: f64, t4180: f64, t4181: f64, t4234: f64, t47277: f64, t58495: f64, t59251: f64, t59255: f64, t59257: f64, t59259: f64, t59261: f64, t67607: f64, t9642: f64) -> f64 {
    let t68186 = -t47277 + t2643 * t2645 * t16891 * t16912 / 256.0_f64 + t9642 * t20887 / 256.0_f64 + t2643 * t2645 * t13242 * t20885 / 256.0_f64 + t2643 * t2645 * t4181 * t232 * t16662 / 256.0_f64 - t9642 * t20891 / 1024.0_f64 - t2643 * t4180 * t58495 * t1510 / 1024.0_f64 - t2643 * t4180 * t16891 * t4234 / 1024.0_f64 + t2643 * t2645 * t67607 * t2647 / 768.0_f64 - 35.0_f64 / 384.0_f64 * t59251 + 7.0_f64 / 384.0_f64 * t59255 + 7.0_f64 / 384.0_f64 * t59257 - 119.0_f64 / 576.0_f64 * t59259 + 7.0_f64 / 192.0_f64 * t59261;
    t68186
}
