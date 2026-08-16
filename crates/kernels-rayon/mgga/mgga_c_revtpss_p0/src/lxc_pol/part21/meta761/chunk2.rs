//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2698/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2698(t1353: f64, t13716: f64, t4139: f64, t4140: f64, t47076: f64, t48281: f64, t48283: f64, t48284: f64, t48286: f64, t48288: f64, t48291: f64, t48293: f64, t48295: f64, t5536: f64, t566: f64) -> f64 {
    let t49611 = 18.0_f64 * t1353 * t13716 * t5536 * t566 + 9.0_f64 * t13716 * t4139 * t4140 - t47076 - t48281 - t48283 - t48284 + t48286 + t48288 - t48291 + t48293 - t48295;
    t49611
}
