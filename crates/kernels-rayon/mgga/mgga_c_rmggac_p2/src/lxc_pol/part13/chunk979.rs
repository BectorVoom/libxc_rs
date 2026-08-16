//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 979/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk979(t25854: f64, t38983: f64, t6444: f64, t9005: f64, t40134: f64, t5259: f64, t39700: f64, t797: f64, t40897: f64, t5271: f64, t40920: f64, t5162: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41492 = t25854 * t38983;
    let t41501 = t6444 * t9005;
    let t41506 = t5259 * t40134;
    let t41523 = t797 * t39700;
    let t41531 = t5271 * t40897;
    let t41534 = t5162 * t40920;
    (t41492, t41501, t41506, t41523, t41531, t41534)
}
