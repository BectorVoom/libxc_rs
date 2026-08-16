//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 987/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk987(t311: f64, t8950: f64, t10: f64, t3145: f64, t2917: f64, t8700: f64, t106: f64, t1141: f64, t116: f64, t3241: f64, t3242: f64, t11899: f64, t2849: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12042 = t311 * t8950;
    let t12068 = t10 * t3145;
    let t12478 = t8700 * t2917;
    let t12532 = t106 * t1141;
    let t12567 = t3241 * t3242 * t116;
    let t12568 = t11899 * t2849;
    (t12042, t12068, t12478, t12532, t12567, t12568)
}
