//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 636/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk636(t511: f64, t8502: f64, t7231: f64, t3351: f64, t1632: f64, t3352: f64, t2313: f64, t458: f64, t1979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8503 = t511 * t8502;
    let t8504 = t7231 * t8503;
    let t8505 = t3351 * t8504;
    let t8507 = t511 * t1632;
    let t8508 = t3352 * t8507;
    let t8509 = t3351 * t8508;
    let t8511 = t2313 * t458;
    let t8512 = t8511 * t1979;
    (t8504, t8505, t8508, t8509, t8511, t8512)
}
