//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 932/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk932(t352: f64, t8924: f64, t262: f64, t8620: f64, t34735: f64, t8902: f64, t36639: f64, t8906: f64, t2412: f64, t7687: f64, t1392: f64, t1979: f64, t1982: f64, t201: f64, t457: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40487 = t8924 * t352;
    let t40488 = t262 * t40487;
    let t40489 = t8620 * t40488;
    let t40491 = t34735 * t8902;
    let t40493 = t36639 * t8906;
    let t40495 = t2412 * t7687;
    let t40502 = t1392 * t457 * t201 * t1979 * t1982;
    (t40487, t40488, t40489, t40491, t40493, t40495, t40502)
}
