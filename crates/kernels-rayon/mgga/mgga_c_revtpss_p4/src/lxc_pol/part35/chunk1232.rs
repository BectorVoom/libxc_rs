//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1232/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1232(t114986: f64, t115406: f64, t115962: f64, t116006: f64, t111320: f64, t114905: f64, t117: f64, t1518: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t22633: f64, t25055: f64, t25063: f64, t25066: f64, t25069: f64, t28986: f64, t30637: f64, t30651: f64, t30654: f64, t30657: f64, t30660: f64, t34359: f64, t572: f64, t573: f64, t5883: f64, t5920: f64, t6941: f64, t6945: f64, t6948: f64, t7553: f64, t7983: f64, t8118: f64, t8124: f64, t8127: f64, param_d: f64) -> (f64, f64) {
    let t116008 = t114986 + t115406 + t115962 + t116006;
    let t116023 = 3.0_f64 * t572 * t117 * t114905 + 18.0_f64 * t8118 * t6945 + 18.0_f64 * t2113 * t25066 + 9.0_f64 * t1916 * t30660 + 9.0_f64 * t8118 * t6948 + 6.0_f64 * t572 * t7553 * t22633 + 6.0_f64 * t2113 * t25063 + 18.0_f64 * t572 * t111320 * t1518 + 18.0_f64 * t572 * t28986 * t5920 + 9.0_f64 * t30637 * t1918 + 18.0_f64 * t1916 * t30651 + 36.0_f64 * t1916 * t30654 + 18.0_f64 * t1916 * t30657 + 18.0_f64 * t6941 * t8124 + param_d * t116008 * t573 + 18.0_f64 * t572 * t34359 * t5920 + 18.0_f64 * t572 * t5883 * t7983 + 9.0_f64 * t6941 * t8127 + 3.0_f64 * t2113 * t25069 + 3.0_f64 * t25055 * t2115;
    (t116008, t116023)
}
