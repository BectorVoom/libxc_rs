//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 619/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk619(t304: f64, t4922: f64, t355: f64, t360: f64, t303: f64, t1699: f64, t2880: f64, t991: f64, t2888: f64, t291: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4923 = t304 * t4922;
    let t4924 = t4923 * t355;
    let t4925 = t4924 * t360;
    let t4926 = t303 * t4925;
    let t4936 = t2880 * t1699;
    let t4937 = t991 * t4936;
    let t4939 = t2888 * t291;
    (t4923, t4924, t4925, t4926, t4936, t4937, t4939)
}
