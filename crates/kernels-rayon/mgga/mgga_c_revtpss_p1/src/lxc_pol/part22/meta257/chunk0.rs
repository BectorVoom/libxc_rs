//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1586/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1586(t1132: f64, t6449: f64, t3407: f64, t6442: f64, t1139: f64, t3417: f64, t6421: f64, t141: f64, t1145: f64, t6425: f64, t6429: f64, t3402: f64, t3414: f64, t5044: f64, t5093: f64, t6423: f64, t6427: f64, t6431: f64, t6443: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6450 = t1132 * t6449;
    let t6456 = t3407 * t6442;
    let t6458 = t1139 * t6449;
    let t6461 = t3417 * t6421;
    let t6462 = t141 * t6461;
    let t6464 = t1145 * t6425;
    let t6465 = t141 * t6464;
    let t6467 = t1145 * t6429;
    let t6468 = t141 * t6467;
    let t6470 = -0.9494625e0_f64 * t6443 + 0.1898925e1_f64 * t6450 + t3402 - 0.19931111111111111111e0_f64 * t5044 - 0.19931111111111111111e0_f64 * t6423 + 0.59793333333333333334e0_f64 * t6427 + 0.29896666666666666667e0_f64 * t6431 + 0.15358125e0_f64 * t6456 + 0.3071625e0_f64 * t6458 + t3414 - 0.10954222222222222222e0_f64 * t5093 - 0.27385555555555555556e-1_f64 * t6462 + 0.16431333333333333333e0_f64 * t6465 + 0.82156666666666666667e-1_f64 * t6468;
    (t6450, t6456, t6458, t6461, t6462, t6464, t6465, t6467, t6468, t6470)
}
