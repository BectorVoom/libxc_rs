//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2876/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2876(t262: f64, t5966: f64, t23148: f64, t23124: f64, t39429: f64, t39432: f64, t39442: f64, t4541: f64, t49877: f64, t50080: f64, t76937: f64, t76938: f64, t76939: f64, t76940: f64, t76941: f64, t775: f64) -> (f64, f64) {
    let t77333 = t5966 * t262;
    let t77341 = t262 * t23148;
    let t77347 = 6.0_f64 * t4541 * t77341 * t775 + 18.0_f64 * t23124 * t50080 - t39429 - t39432 + t39442 + t49877 - t76937 - t76938 - t76939 + t76940 + t76941;
    (t77333, t77347)
}
