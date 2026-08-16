//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1197/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1197(t2425: f64, t7603: f64, t86: f64, t26439: f64, t748: f64, t26446: f64, t2480: f64, t137: f64, t8955: f64, t2490: f64, t2526: f64, t752: f64, t774: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91932 = t86 * t2425 * t7603;
    let t91935 = t86 * t748 * t26439;
    let t91938 = t86 * t748 * t26446;
    let t91941 = t86 * t2480 * t7603;
    let t91944 = t86 * t8955 * t137;
    let t91948 = t752 * t2490 * t2526 * t774;
    (t91932, t91935, t91938, t91941, t91944, t91948)
}
