//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 368/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk368(t227: f64, t2063: f64, t565: f64, t806: f64, t695: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t2359 = piecewise3(t228, 0.0_f64, t2063);
    let t2360 = t565 * t2359;
    let t2361 = t2360 * t806;
    let t2364 = t695 * t2063;
    (t2360, t2361, t2364)
}
