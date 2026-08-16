//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1091/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1091(t10293: f64, t29664: f64, t33498: f64, t3437: f64, t11449: f64, t11815: f64, t190: f64, t2786: f64, t33374: f64, t7595: f64, t15553: f64, t15555: f64, t33287: f64) -> (f64, f64, f64, f64) {
    let t33501 = t3437 * t33498 * t10293 * t29664;
    let t33505 = t2786 * t190 * t11449 * t11815;
    let t33507 = t33374 * t7595;
    let t33510 = t15553 * t33287 * t15555;
    (t33501, t33505, t33507, t33510)
}
