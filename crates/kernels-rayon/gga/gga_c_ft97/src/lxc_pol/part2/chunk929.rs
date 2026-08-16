//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 929/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk929(t1273: f64, t2961: f64, t4381: f64, t2956: f64, t4375: f64, t909: f64, t332: f64, t505: f64, t4380: f64, t2957: f64, t992: f64, t4354: f64, t8675: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14390 = t1273 * t2961;
    let t14391 = t14390 * t4381;
    let t14394 = t1273 * t2956;
    let t14395 = t14394 * t4381;
    let t14402 = t4375 * t909;
    let t14403 = t14402 * t4381;
    let t14408 = t332 * t505;
    let t14409 = t4380 * t14408;
    let t14412 = t2957 * t992;
    let t14421 = 4.0_f64 / 9.0_f64 * t8675 * t4354;
    (t14391, t14395, t14403, t14409, t14412, t14421)
}
