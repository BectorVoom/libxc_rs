//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1205/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1205(t20151: f64, t5078: f64, t19945: f64, t19948: f64, t19951: f64, t19954: f64, t19958: f64, t19961: f64, t19963: f64, t19965: f64, t19967: f64, t19970: f64, t20131: f64, t20134: f64, t20137: f64, t20139: f64, t20143: f64, t20146: f64, t20149: f64) -> (f64, f64) {
    let t20152 = t20151 * t5078;
    let t20154 = t19945 / 96.0_f64 + t19948 / 864.0_f64 + t19951 / 12.0_f64 + t19954 / 8.0_f64 + t19958 / 24.0_f64 - t19961 / 64.0_f64 + t19963 / 128.0_f64 - t19965 / 72.0_f64 - t19967 / 96.0_f64 - t19970 / 9.0_f64 + t20131 / 16.0_f64 - t20134 / 3.0_f64 + t20137 / 4.0_f64 - 2.0_f64 / 9.0_f64 * t20139 - t20143 / 16.0_f64 + t20146 / 256.0_f64 + t20149 / 6.0_f64 - t20152 / 36.0_f64;
    (t20152, t20154)
}
