//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1118/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1118(t2099: f64, t6508: f64, t918: f64, t17928: f64, t2362: f64, t326: f64, t17932: f64, t401: f64, t913: f64, t2367: f64, t2372: f64, t5939: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19076 = t918 * t2099 * t6508;
    let t19078 = t17928 * t2362;
    let t19079 = t19078 * t326;
    let t19080 = t401 * t17932;
    let t19090 = t17928 * t913;
    let t19091 = t19090 * t326;
    let t19099 = t2367 * t5939 * t2372;
    (t19076, t19078, t19079, t19080, t19090, t19091, t19099)
}
