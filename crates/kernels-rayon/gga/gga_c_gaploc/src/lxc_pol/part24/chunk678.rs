//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 678/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk678(t6134: f64, t789: f64, t1980: f64, t2026: f64, t2177: f64, t832: f64, t161: f64, t2299: f64, t1353: f64, t1359: f64, t3176: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6135 = t6134 * t789;
    let t6138 = t1980 * t2026;
    let t6159 = t2177 * t832;
    let t6285 = t2299 * t161;
    let t6286 = t6285 * t1353;
    let t6289 = t1359 * t3176;
    let t6290 = t6289 * t488;
    (t6135, t6138, t6159, t6286, t6289, t6290)
}
