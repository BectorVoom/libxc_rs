//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1189/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1189(t11002: f64, t41189: f64, t3269: f64, t1044: f64, t11323: f64, t41153: f64, t41156: f64, t41158: f64, t41160: f64, t41162: f64, t41165: f64, t41168: f64, t41170: f64, t41173: f64, t41176: f64, t41179: f64, t41182: f64, t41185: f64, t41188: f64) -> (f64, f64) {
    let t41190 = t11002 * t41189;
    let t41192 = 5.0_f64 / 8.0_f64 * t3269 * t41190;
    let t41193 = t1044 * t11323 - t41153 - t41156 + t41158 + t41160 + t41162 + t41165 - t41168 - t41170 - t41173 - t41176 - t41179 - t41182 - t41185 + t41188 - t41192;
    (t41192, t41193)
}
