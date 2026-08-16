//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 823/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk823(t256: f64, t7341: f64, t2476: f64, t7342: f64, t7604: f64, t837: f64, t2485: f64, t809: f64, t2517: f64, t805: f64, t2529: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7813 = t256 * t7341;
    let t7814 = t7342 * t2476;
    let t7817 = t7604 * t837;
    let t7820 = t2485 * t809;
    let t7825 = t805 * t2517;
    let t7828 = t824 * t2529;
    (t7813, t7814, t7817, t7820, t7825, t7828)
}
