//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1465/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1465(t41316: f64, t41323: f64, t41353: f64, t41356: f64, t41359: f64, t41396: f64, t41402: f64, t41404: f64, t41406: f64, t41409: f64, t41412: f64, t41414: f64, t41417: f64, t41419: f64) -> f64 {
    let t41717 = -0.185931e2_f64 * t41316 + 0.123954e2_f64 * t41323 - 0.34431666666666666667e1_f64 * t41353 + 0.41318e1_f64 * t41356 - 0.13772666666666666667e1_f64 * t41359 + 0.3529725e1_f64 * t41396 - 0.6618234375e1_f64 * t41402 - 0.52945875e1_f64 * t41404 + 0.27785333333333333333e0_f64 * t41406 - 0.375102e1_f64 * t41409 + 0.158837625e2_f64 * t41412 - 0.705945e1_f64 * t41414 - 0.94674375e0_f64 * t41417 + 0.1262325e1_f64 * t41419;
    t41717
}
