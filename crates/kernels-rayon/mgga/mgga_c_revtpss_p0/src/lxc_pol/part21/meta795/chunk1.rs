//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2876/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2876(t11396: f64, t4719: f64, t15566: f64, t5023: f64, t52170: f64, t52174: f64, t52176: f64, t52178: f64, t52180: f64, t52182: f64, t52185: f64, t52187: f64, t52188: f64, t52194: f64) -> (f64, f64) {
    let t52196 = 0.51947577317044391277e2_f64 * t4719 * t11396;
    let t52197 = 6.0_f64 * t15566 * t5023 * t52188 + t52170 + t52174 - t52176 - t52178 + t52180 + t52182 - t52185 - t52187 + t52194 - t52196;
    (t52196, t52197)
}
