//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1295/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1295(t56986: f64, t57007: f64, t57052: f64, t57108: f64, t779: f64, t799: f64, t4818: f64, t24302: f64, t24305: f64, t16817: f64, t3793: f64, t845: f64) -> (f64, f64, f64, f64) {
    let t57113 = 1.0_f64 * t779 * (t56986 + t57007 + t57052 + t57108) * t799;
    let t57114 = t4818 * t4818;
    let t57117 = 0.24954977986735470917e5_f64 * t24302 * t57114 * t24305;
    let t57120 = 0.46785787179641632568e1_f64 * t845 * t3793 * t16817;
    (t57113, t57114, t57117, t57120)
}
