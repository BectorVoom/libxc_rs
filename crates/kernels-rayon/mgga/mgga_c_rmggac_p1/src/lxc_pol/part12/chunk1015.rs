//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1015/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1015(t25529: f64, t36: f64, t5169: f64, t41027: f64, t851: f64, t2118: f64, t41032: f64, t22: f64, t2353: f64, t26531: f64, t5184: f64, t649: f64) -> (f64, f64, f64, f64, f64) {
    let t41262 = t25529 * t36;
    let t41263 = t41262 * t5169;
    let t41265 = t851 * t41027;
    let t41271 = t2118 * t41032;
    let t41274 = t26531 * t22 * t2353;
    let t41276 = t649 * t5184;
    (t41263, t41265, t41271, t41274, t41276)
}
