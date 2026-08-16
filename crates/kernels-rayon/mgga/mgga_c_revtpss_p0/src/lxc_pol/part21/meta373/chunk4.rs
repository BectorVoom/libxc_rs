//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1773/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1773(t1145: f64, t12277: f64, t141: f64, t3362: f64, t606: f64, t2258: f64) -> (f64, f64, f64) {
    let t12278 = t1145 * t12277;
    let t12279 = t141 * t12278;
    let t12281 = t3362 * t606;
    let t12282 = t12281 * t2258;
    (t12278, t12279, t12282)
}
