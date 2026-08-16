//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 809/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk809(t3438: f64, t4984: f64, t3437: f64, t284: f64, t374: f64, t3217: f64, t41: f64, t4813: f64, t5044: f64, t5049: f64, t5051: f64, t5054: f64, t5056: f64, t5058: f64, t5060: f64, t5063: f64, t5065: f64, t5069: f64, t5071: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5073 = t3438 * t4984;
    let t5074 = t3437 * t5073;
    let t5076 = t374 * t284;
    let t5077 = t41 * t3217;
    let t5078 = t5077 * t4813;
    let t5079 = t5076 * t5078;
    let t5081 = -t5044 / 16.0_f64 + t5049 / 8.0_f64 - t5051 / 192.0_f64 + t5054 / 6.0_f64 - t5056 / 6.0_f64 + t5058 / 24.0_f64 + t5060 / 24.0_f64 - t5063 / 24.0_f64 - t5065 / 192.0_f64 + t5069 / 256.0_f64 - t5071 / 16.0_f64 + t5074 / 192.0_f64 - t5079 / 72.0_f64;
    (t5073, t5074, t5076, t5077, t5078, t5079, t5081)
}
