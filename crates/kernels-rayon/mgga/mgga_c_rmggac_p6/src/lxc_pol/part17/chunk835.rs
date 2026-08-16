//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 835/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk835(t41027: f64, t793: f64, t41035: f64, t797: f64, t41055: f64, t851: f64, t854: f64, t3810: f64, t40920: f64, t41031: f64, t25529: f64, t36: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41191 = t793 * t41027;
    let t41195 = t797 * t41035;
    let t41230 = t851 * t41055;
    let t41231 = 0.17701538806747441785e-2_f64 * t41230;
    let t41233 = t854 * t41035;
    let t41234 = 0.21241846568096930142e-2_f64 * t41233;
    let t41241 = t3810 * t40920;
    let t41242 = 0.14869292597667851099e-1_f64 * t41241;
    let t41247 = t854 * t41031;
    let t41257 = t797 * t41031;
    let t41262 = t25529 * t36;
    (t41191, t41195, t41231, t41234, t41242, t41247, t41257, t41262)
}
