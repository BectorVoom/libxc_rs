//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 962/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk962(t2186: f64, t8597: f64, t2412: f64, t7404: f64, t352: f64, t8924: f64, t262: f64, t8620: f64, t34735: f64, t8902: f64, t36639: f64, t8906: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40479 = t2186 * t8597;
    let t40480 = 0.19863479950205658386e-4_f64 * t40479;
    let t40481 = t2412 * t7404;
    let t40487 = t8924 * t352;
    let t40488 = t262 * t40487;
    let t40489 = t8620 * t40488;
    let t40491 = t34735 * t8902;
    let t40493 = t36639 * t8906;
    (t40480, t40481, t40487, t40488, t40489, t40491, t40493)
}
