//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 836/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk836(t2115: f64, t41028: f64, t36188: f64, t36190: f64, t6444: f64, t8708: f64, t41055: f64, t793: f64, t2100: f64, t41056: f64, t2103: f64, t41036: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41365 = t2115 * t41028;
    let t41367 = 0.64905642291407286545e-2_f64 * t36188;
    let t41368 = 0.77886770749688743854e-2_f64 * t36190;
    let t41371 = t6444 * t8708;
    let t41373 = t793 * t41055;
    let t41377 = t2100 * t41056;
    let t41379 = t2103 * t41036;
    (t41365, t41367, t41368, t41371, t41373, t41377, t41379)
}
