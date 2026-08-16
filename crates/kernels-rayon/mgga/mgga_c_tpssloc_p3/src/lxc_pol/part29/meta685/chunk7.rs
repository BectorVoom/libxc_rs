//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2345/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2345(t1860: f64, t2109: f64, t2110: f64, t22489: f64, t22493: f64, t22534: f64, t24504: f64, t24511: f64, t26024: f64, t27308: f64, t27311: f64, t6486: f64, t7255: f64, t7428: f64, t7445: f64, t7974: f64, t7975: f64, t7978: f64, t90132: f64, t90257: f64) -> f64 {
    let t96209 = t22534 * t7975 / 3.0_f64 + t22534 * t7978 / 3.0_f64 + t90132 * t2110 / 3.0_f64 - t7428 * t24511 / 6.0_f64 - t22493 * t7975 / 6.0_f64 - t1860 * t7974 * t22489 / 6.0_f64 - t22493 * t7978 / 6.0_f64 - t6486 * t27308 / 3.0_f64 - t6486 * t27311 / 3.0_f64 - t1860 * t24504 * t7445 / 6.0_f64 - t1860 * t7255 * t26024 / 3.0_f64 - t1860 * t2109 * t90257 / 6.0_f64;
    t96209
}
