//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 958/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk958(t5978: f64, t827: f64, t828: f64, t124: f64, t5962: f64, t800: f64, t5966: f64, t2477: f64, t190: f64, t5825: f64, t706: f64, t5819: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5980 = t827 * t828 * t5978;
    let t5984 = t124 * t5962;
    let t5985 = t800 * t5984;
    let t5988 = t124 * t5966;
    let t5989 = t800 * t5988;
    let t5993 = t2477 * t828 * t5966;
    let t5999 = t190 * t5825;
    let t6001 = 4.0_f64 * t706 * t5999;
    let t6002 = t190 * t5819;
    (t5980, t5984, t5985, t5988, t5989, t5993, t5999, t6001, t6002)
}
