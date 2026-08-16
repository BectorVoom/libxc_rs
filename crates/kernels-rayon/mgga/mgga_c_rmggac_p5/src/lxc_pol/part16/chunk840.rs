//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 840/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk840(t40921: f64, t8630: f64, t40898: f64, t7198: f64, t36511: f64, t36513: f64, t16156: f64, t9055: f64, t2085: f64, t8339: f64, t1692: f64, t2046: f64, t2050: f64, t31: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41637 = t8630 * t40921;
    let t41641 = t7198 * t40898;
    let t41647 = 0.19863479950205658386e-3_f64 * t36511;
    let t41648 = 0.19863479950205658386e-3_f64 * t36513;
    let t41654 = t16156 * t9055;
    let t41656 = t8339 * t2085;
    let t41667 = t2046 * t2050 * t1692 * t31;
    (t41637, t41641, t41647, t41648, t41654, t41656, t41667)
}
