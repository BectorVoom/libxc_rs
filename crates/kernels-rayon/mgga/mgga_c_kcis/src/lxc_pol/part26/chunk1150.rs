//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1150/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1150(t2002: f64, t28524: f64, t303: f64, t1983: f64, t2012: f64, t7086: f64, t7914: f64, t6176: f64, t15955: f64, t2011: f64, t27387: f64, t1464: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29337 = t28524 * t2002;
    let t29338 = t303 * t29337;
    let t29340 = t1983 * t2012;
    let t29341 = t303 * t29340;
    let t29343 = t7914 * t7086;
    let t29344 = t6176 * t29343;
    let t29353 = t15955 * t2011;
    let t29354 = t27387 * t29353;
    let t29355 = t1464 * t29354;
    (t29337, t29338, t29340, t29341, t29343, t29344, t29353, t29354, t29355)
}
