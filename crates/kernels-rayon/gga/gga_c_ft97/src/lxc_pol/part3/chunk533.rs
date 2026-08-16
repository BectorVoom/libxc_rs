//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 533/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk533(t4191: f64, t856: f64, t91: f64, t1228: f64, t1775: f64, t2: f64, t2766: f64, t3691: f64, t2771: f64, t4037: f64, t848: f64, t3700: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4193 = t91 * t4191 * t856;
    let t4197 = t1775 * t1228;
    let t4199 = t2766 * t2;
    let t4200 = t4199 * t3691;
    let t4203 = t2771 * t4037;
    let t4206 = t848 * t2;
    let t4207 = t4206 * t3700;
    (t4193, t4197, t4199, t4200, t4203, t4206, t4207)
}
