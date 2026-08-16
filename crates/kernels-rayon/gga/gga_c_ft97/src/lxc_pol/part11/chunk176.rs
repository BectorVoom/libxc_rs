//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 176/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk176(t358: f64, t422: f64, t363: f64, t420: f64, t419: f64, t412: f64, t417: f64) -> (f64, f64, f64, f64, f64) {
    let t423 = t422 * t358;
    let t424 = t423 * t363;
    let t425 = t420 * t424;
    let t426 = t419 * t425;
    let t428 = -0.51074886703703703704e-1_f64 * t412 + t417 + 0.6384360837962962963e-2_f64 * t426;
    (t423, t424, t425, t426, t428)
}
