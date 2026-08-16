//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3612/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3612(t3391: f64, t43821: f64, t6442: f64, t12327: f64, t6449: f64, t43946: f64, t12331: f64, t16926: f64, t5071: f64, t1134: f64, t20337: f64, t3390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68470 = t43821 * t6442 * t3391;
    let t68473 = t12327 * t6449 * t3391;
    let t68476 = t43946 * t6442 * t3391;
    let t68479 = t12331 * t6449 * t3391;
    let t68481 = t5071 * t16926;
    let t68484 = t3390 * t20337 * t1134;
    (t68470, t68473, t68476, t68479, t68481, t68484)
}
