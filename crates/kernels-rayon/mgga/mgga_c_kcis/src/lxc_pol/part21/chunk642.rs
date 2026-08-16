//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 642/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk642(t5076: f64, t5078: f64, t5044: f64, t5049: f64, t5051: f64, t5054: f64, t5056: f64, t5058: f64, t5060: f64, t5063: f64, t5065: f64, t5069: f64, t5071: f64, t5074: f64) -> (f64, f64) {
    let t5079 = t5076 * t5078;
    let t5081 = -t5044 / 16.0_f64 + t5049 / 8.0_f64 - t5051 / 192.0_f64 + t5054 / 6.0_f64 - t5056 / 6.0_f64 + t5058 / 24.0_f64 + t5060 / 24.0_f64 - t5063 / 24.0_f64 - t5065 / 192.0_f64 + t5069 / 256.0_f64 - t5071 / 16.0_f64 + t5074 / 192.0_f64 - t5079 / 72.0_f64;
    (t5079, t5081)
}
