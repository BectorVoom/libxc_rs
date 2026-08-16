//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1050/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1050(t11547: f64, t11552: f64, t11564: f64, t11567: f64, t11570: f64, t11572: f64, t11574: f64, t11581: f64, t11584: f64, t11599: f64, t11602: f64, t11605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12129 = 0.1422820120100248667e-7_f64 * t11547;
    let t12131 = 0.11594181388521408695e-4_f64 * t11552;
    let t12135 = 0.11594181388521408695e-4_f64 * t11564;
    let t12136 = 0.21720231316129303386e-4_f64 * t11567;
    let t12137 = 0.2318836277704281739e-4_f64 * t11570;
    let t12138 = 0.21720231316129303386e-4_f64 * t11572;
    let t12139 = 0.34752370105806885418e-3_f64 * t11574;
    let t12140 = 0.28960308421505737848e-5_f64 * t11581;
    let t12141 = 0.1349435763888888889e-4_f64 * t11584;
    let t12144 = 0.67530371184977617164e-6_f64 * t11599;
    let t12145 = 0.13506074236995523433e-5_f64 * t11602;
    let t12146 = 0.21103240995305505364e-7_f64 * t11605;
    (t12129, t12131, t12135, t12136, t12137, t12138, t12139, t12140, t12141, t12144, t12145, t12146)
}
