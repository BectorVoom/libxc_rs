//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 932/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk932(t14450: f64, t2923: f64, t1934: f64, t4352: f64, t1268: f64, t2347: f64, t10845: f64, t2349: f64, t1091: f64, t2951: f64, t3746: f64, t904: f64) -> (f64, f64, f64, f64, f64) {
    let t14451 = t2923 * t14450;
    let t14455 = t2923 * t4352 * t1934;
    let t14458 = t1268 * t2347;
    let t14460 = t10845 * t14458 * t2349;
    let t14464 = t2923 * t1091 * t2951;
    let t14468 = t2923 * t3746 * t904;
    (t14451, t14455, t14460, t14464, t14468)
}
