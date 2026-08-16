//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1460/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1460(t41316: f64, t41323: f64, t41353: f64, t41356: f64, t41359: f64, t41396: f64, t41402: f64, t41404: f64, t41406: f64, t41409: f64, t41412: f64, t41414: f64, t41417: f64, t41419: f64) -> f64 {
    let t41637 = -0.107628e2_f64 * t41316 + 0.71752000000000000001e1_f64 * t41323 - 0.19931111111111111111e1_f64 * t41353 + 0.23917333333333333333e1_f64 * t41356 - 0.79724444444444444444e0_f64 * t41359 + 0.1898925e1_f64 * t41396 - 0.3560484375e1_f64 * t41402 - 0.28483875e1_f64 * t41404 + 0.21908444444444444444e0_f64 * t41406 - 0.295764e1_f64 * t41409 + 0.85451625e1_f64 * t41412 - 0.379785e1_f64 * t41414 - 0.46074375e0_f64 * t41417 + 0.614325e0_f64 * t41419;
    t41637
}
