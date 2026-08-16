//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 313/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk313(t2: f64, t2766: f64, t3691: f64, t2771: f64, t4037: f64, t848: f64, t3700: f64, t3921: f64, t1232: f64, t458: f64, t4052: f64, t1212: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4199 = t2766 * t2;
    let t4200 = t4199 * t3691;
    let t4203 = t2771 * t4037;
    let t4206 = t848 * t2;
    let t4207 = t4206 * t3700;
    let t4210 = t848 * t3921;
    let t4213 = t458 * t1232;
    let t4215 = t2771 * t4052;
    let t4218 = t2 * t1212;
    (t4200, t4203, t4207, t4210, t4213, t4215, t4218)
}
