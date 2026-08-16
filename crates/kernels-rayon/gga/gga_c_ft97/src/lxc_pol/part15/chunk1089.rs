//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1089/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1089(t1053: f64, t77633: f64, t144: f64, t167: f64, t1901: f64, t20868: f64, t2185: f64, t2210: f64, t2221: f64, t3440: f64, t446: f64, t49622: f64, t50679: f64, t64001: f64, t76618: f64, t77491: f64, t77505: f64, t77521: f64, t86681: f64, t87091: f64, t87295: f64, t925: f64) -> (f64, f64) {
    let t87699 = t77633 * t1053;
    let t87707 = -4.0_f64 / 3.0_f64 * t77491 + 8.0_f64 / 3.0_f64 * t77505 + 2.0_f64 * t446 * t2185 * t167 * t86681 + 16.0_f64 / 9.0_f64 * t64001 + 8.0_f64 / 3.0_f64 * t77521 - 2.0_f64 * t446 * t144 * t87091 + 8.0_f64 / 3.0_f64 * t1901 * t2221 * t3440 * t87295 + 8.0_f64 / 3.0_f64 * t1901 * t2210 * t76618 * t925 - 4.0_f64 / 3.0_f64 * t446 * t144 * t87699 + 112.0_f64 / 81.0_f64 * t50679 - 8.0_f64 / 3.0_f64 * t1901 * t49622 * t20868;
    (t87699, t87707)
}
