//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1766/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1766(t12254: f64, t12257: f64, t141: f64, t1146: f64, t2439: f64) -> (f64, f64, f64) {
    let t12258 = t12254 * t12257;
    let t12259 = t141 * t12258;
    let t12261 = t2439 * t1146;
    (t12258, t12259, t12261)
}
