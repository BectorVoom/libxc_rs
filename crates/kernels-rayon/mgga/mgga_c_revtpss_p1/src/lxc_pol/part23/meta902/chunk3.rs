//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2882/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2882(t5962: f64, t890: f64, t1544: f64, t18850: f64, t18860: f64, t18865: f64, t2403: f64, t27375: f64, t4343: f64, t4433: f64, t4541: f64, t4556: f64, t50866: f64, t63146: f64, t77012: f64, t77013: f64, t77014: f64, t77015: f64, t77020: f64) -> f64 {
    let t77425 = t5962 * t890;
    let t77429 = 18.0_f64 * t1544 * t4541 * t63146 + 18.0_f64 * t18850 * t4433 * t4541 + 18.0_f64 * t18860 * t4343 * t4541 - 9.0_f64 * t18865 * t2403 * t27375 - 9.0_f64 * t2403 * t4556 * t77425 + t50866 - t77012 - t77013 - t77014 + t77015 + t77020;
    t77429
}
