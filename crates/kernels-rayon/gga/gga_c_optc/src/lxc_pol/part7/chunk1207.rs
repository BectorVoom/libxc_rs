//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1207/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1207(t2476: f64, t7604: f64, t7676: f64, t7692: f64, t7629: f64, t7686: f64, t2373: f64, t7664: f64, t798: f64, t2416: f64, t2418: f64, t7663: f64) -> (f64, f64, f64, f64, f64) {
    let t24950 = t7604 * t2476;
    let t24955 = 24.0_f64 * t7676 * t7692;
    let t24957 = 0.19298189186581325787e3_f64 * t7629 * t7686;
    let t24960 = 8.0_f64 * t2373 * t7664 * t798;
    let t24964 = 0.64327297288604419288e2_f64 * t2416 * t7663 * t2418 * t798;
    (t24950, t24955, t24957, t24960, t24964)
}
