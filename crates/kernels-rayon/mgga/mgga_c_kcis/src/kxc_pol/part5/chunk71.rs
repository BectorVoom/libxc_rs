//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 71/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk71(t174: f64, t176: f64, t37: f64, t173: f64, t44: f64, t41: f64, zeta_threshold: f64) -> (f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t178 = piecewise3(t175, t37, t176 * t174);
    let t180 = (t173 + t178 - 2.0_f64) * t44;
    let t183 = piecewise3(2.0_f64 <= zeta_threshold, t37, 2.0_f64 * t41);
    let t185 = piecewise3(0.0_f64 <= zeta_threshold, t37, 0.0_f64);
    let t187 = (t183 + t185 - 2.0_f64) * t44;
    (t180, t187)
}
