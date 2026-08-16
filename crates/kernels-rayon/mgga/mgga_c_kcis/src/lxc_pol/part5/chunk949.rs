//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 949/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk949(t9280: f64, t9024: f64, t9026: f64, t9028: f64, t9031: f64, t9034: f64, t9036: f64, t9038: f64, t9040: f64, t9043: f64, t9048: f64, t9050: f64, t9054: f64, t9056: f64, t9058: f64) -> (f64, f64) {
    let t9281 = 6.0_f64 * t9280;
    let t9296 = 9.0_f64 / 4.0_f64 * t9024 - 15.0_f64 / 16.0_f64 * t9026 + 3.0_f64 / 2.0_f64 * t9028 - 3.0_f64 / 16.0_f64 * t9031 + 15.0_f64 / 16.0_f64 * t9034 - 9.0_f64 / 4.0_f64 * t9036 - 3.0_f64 / 8.0_f64 * t9038 + 3.0_f64 / 16.0_f64 * t9040 + 3.0_f64 / 4.0_f64 * t9043 - 3.0_f64 / 32.0_f64 * t9048 - 3.0_f64 / 32.0_f64 * t9050 + 3.0_f64 / 4.0_f64 * t9054 - 3.0_f64 * t9056 + 3.0_f64 / 64.0_f64 * t9058;
    (t9281, t9296)
}
