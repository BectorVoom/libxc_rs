//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 765/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk765(t311: f64, t8950: f64, t10: f64, t3145: f64, t1506: f64, t8446: f64, t8487: f64, t1516: f64, t3138: f64, t1508: f64, t3137: f64, t1121: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12042 = t311 * t8950;
    let t12068 = t10 * t3145;
    let t12075 = t8446 * t1506;
    let t12079 = t8487 * t1506;
    let t12098 = t1516 * t3138;
    let t12105 = t3137 * t1508;
    let t12106 = t1121 * t12105;
    (t12042, t12068, t12075, t12079, t12098, t12105, t12106)
}
