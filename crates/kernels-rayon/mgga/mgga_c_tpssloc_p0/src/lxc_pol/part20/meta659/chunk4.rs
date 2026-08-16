//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2456/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2456(t10937: f64, t13765: f64, t3040: f64, t607: f64, t883: f64, t1023: f64, t10957: f64, t10962: f64, t14211: f64, t14215: f64, t3070: f64, t3071: f64, t42388: f64, t42483: f64, t42505: f64, t43246: f64, t43248: f64, t43253: f64, t43254: f64, t43361: f64, t4337: f64, t4585: f64, t4590: f64, t4652: f64, t48611: f64, t48612: f64, t49616: f64, t49976: f64) -> f64 {
    let t50272 = t10937 * t13765;
    let t50281 = t3040 * t883 * t607;
    let t50301 = t10962 * t4652 / 1024.0_f64 - t50272 / 216.0_f64 + t42483 * t48611 * t49616 * t1023 / 1024.0_f64 - t43246 / 288.0_f64 - t43248 / 648.0_f64 - t43253 + t42388 * t3071 * t48612 * t50281 / 256.0_f64 - t43361 * t3071 * t14211 * t50281 / 256.0_f64 - t42505 * t14215 / 72.0_f64 + t3070 * t3071 * t4337 * t49976 / 256.0_f64 - 19.0_f64 / 432.0_f64 * t10957 * t4585 + 95.0_f64 / 2592.0_f64 * t10957 * t4590 - t43254 / 288.0_f64;
    t50301
}
