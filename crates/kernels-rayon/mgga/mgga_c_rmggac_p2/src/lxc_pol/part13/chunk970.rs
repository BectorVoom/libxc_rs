//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 970/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk970(t2118: f64, t41032: f64, t22: f64, t2353: f64, t26531: f64, t5184: f64, t649: f64, t8746: f64, t41209: f64, t8750: f64, t41212: f64, t41215: f64, t7603: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41271 = t2118 * t41032;
    let t41274 = t26531 * t22 * t2353;
    let t41276 = t649 * t5184;
    let t41277 = t8746 * t41276;
    let t41279 = t8750 * t41209;
    let t41281 = t8750 * t41212;
    let t41283 = t7603 * t41215;
    (t41271, t41274, t41276, t41277, t41279, t41281, t41283)
}
