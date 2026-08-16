//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1301/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1301(t100666: f64, t100669: f64, t100672: f64, t100678: f64, t101043: f64, t101047: f64, t27812: f64, t93590: f64, t96138: f64, t96148: f64, t96150: f64, t96173: f64) -> f64 {
    let t101457 = -t96138 - t93590 + 0.88437037037037037035e-2_f64 * t100666 + 0.66327777777777777776e-2_f64 * t100669 - t96148 - 0.30891203703703703704e-3_f64 * t96150 + 0.16581944444444444444e-2_f64 * t100672 - 0.92673611111111111112e-3_f64 * t96173 - 0.24872916666666666666e-2_f64 * t100678 + 0.185671721767578125e-4_f64 * t27812 * t101043 + 0.111403033060546875e-3_f64 * t27812 * t101047;
    t101457
}
