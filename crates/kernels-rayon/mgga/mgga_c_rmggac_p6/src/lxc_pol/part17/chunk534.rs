//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 534/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk534(t1973: f64, t7244: f64, t1004: f64, t108: f64, t490: f64) -> (f64, f64) {
    let t7245 = t7244 * t1973;
    let t7246 = 0.19863479950205658386e-4_f64 * t7245;
    let t7247 = t1004 * t108;
    let t7248 = t490 * t7247;
    (t7246, t7248)
}
