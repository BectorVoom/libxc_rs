//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2343/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2343(t20947: f64, t776: f64, t13005: f64, t13222: f64, t13223: f64, t13251: f64, t13350: f64, t13365: f64, t16907: f64, t16985: f64, t20885: f64, t20972: f64, t221: f64, t2643: f64, t41096: f64, t4172: f64, t4191: f64, t4255: f64, t5617: f64, t5628: f64, t58642: f64, t58791: f64, t58797: f64, t58809: f64, t58845: f64, t58847: f64) -> (f64, f64) {
    let t68010 = t20947 * t776;
    let t68018 = -7.0_f64 / 192.0_f64 * t58791 + 7.0_f64 / 96.0_f64 * t58797 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t13223 * t20972 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t5617 * t4255 + t41096 + 119.0_f64 / 2304.0_f64 * t58809 + t2643 * t13222 * t13223 * t20885 / 256.0_f64 + 7.0_f64 / 384.0_f64 * t58845 + 7.0_f64 / 192.0_f64 * t58847 + t13251 * t16907 / 256.0_f64 + t58642 * t4191 / 256.0_f64 - 3.0_f64 / 4.0_f64 * t13005 * t221 * t68010 - t13365 * t5628 / 256.0_f64 - t4172 * t16985 / 256.0_f64;
    (t68010, t68018)
}
