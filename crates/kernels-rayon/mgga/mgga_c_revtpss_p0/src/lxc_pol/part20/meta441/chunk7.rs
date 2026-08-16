//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1682/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1682(t1298: f64, t1300: f64, t13190: f64, t198: f64, t336: f64, t3801: f64, t44096: f64, t44100: f64, t44103: f64, t44106: f64, t44108: f64, t44111: f64, t44114: f64, t44122: f64, t44123: f64, t44126: f64, t44984: f64, t44987: f64, t45448: f64, t45494: f64, t45544: f64, t45895: f64, t5023: f64) -> f64 {
    let t45901 = t44096 + t44100 - t44103 + t44106 + t44108 - t44111 - t44114 - 4.0_f64 * t5023 * t13190 * t3801 * t1298 + t44122 - 6.0_f64 * t198 * t336 * t44123 * t44126 + t198 * t336 * (t45448 + t45494 + t45544 + t45895) * t1300 + t44984 - t44987;
    t45901
}
