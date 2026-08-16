//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2337/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2337(t1484: f64, t4233: f64, t5544: f64, t828: f64, t13222: f64, t13228: f64, t13350: f64, t13351: f64, t1510: f64, t16944: f64, t16949: f64, t20969: f64, t2618: f64, t2643: f64, t4178: f64, t4255: f64, t46577: f64, t5585: f64, t5591: f64, t5611: f64, t58550: f64, t58569: f64, t58574: f64, t67568: f64, t776: f64, t817: f64, t819: f64, t820: f64) -> f64 {
    let t67783 = t1484 * t4233;
    let t67793 = t5544 * t828;
    let t67826 = 595.0_f64 / 864.0_f64 * t46577 + 5.0_f64 / 128.0_f64 * t4178 * t13350 * t5585 * t4255 + t2643 * t13222 * t1510 * t67783 / 128.0_f64 - 3.0_f64 / 128.0_f64 * t4178 * t13222 * t5585 * t13351 - 35.0_f64 / 72.0_f64 * t58550 - t4178 * t13222 * t13228 * t67793 / 128.0_f64 - t4178 * t13222 * t13228 * t5611 * t776 / 128.0_f64 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t1510 * t16949 - t4178 * t13222 * t13228 * t67783 / 64.0_f64 - t2618 * t20969 / 3072.0_f64 - t817 * t819 * t820 * t67568 / 3072.0_f64 - 5.0_f64 / 128.0_f64 * t2643 * t13350 * t1510 * t16944 + t2643 * t13222 * t58569 * t5591 / 256.0_f64 + 595.0_f64 / 1152.0_f64 * t58574;
    t67826
}
