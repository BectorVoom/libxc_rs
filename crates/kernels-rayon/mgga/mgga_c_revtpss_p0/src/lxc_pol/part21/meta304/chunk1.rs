//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1560/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1560(t207: f64, t242: f64, t240: f64, t72: f64, t10627: f64, t828: f64, t136: f64, t2476: f64) -> (f64, f64, f64, f64, f64) {
    let t10696 = 1.0_f64 / t242 / t207;
    let t10697 = t240 * t10696;
    let t10698 = t10697 * t72;
    let t10700 = t10698 * t828 * t10627;
    let t10703 = t2476 * t136;
    (t10696, t10697, t10698, t10700, t10703)
}
