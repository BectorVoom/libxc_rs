//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1119/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1119(t17891: f64, t17899: f64, t26416: f64, t291: f64, t5542: f64, t1089: f64, t3687: f64, t9906: f64, t11945: f64, t9895: f64, t11878: f64, t15805: f64, t1936: f64) -> (f64, f64, f64, f64) {
    let t33847 = t17891 * t5542 * t26416 * t291 * t17899;
    let t33850 = t9906 * t3687 * t1089;
    let t33852 = t9895 * t11945;
    let t33855 = t15805 * t1936 * t11878;
    (t33847, t33850, t33852, t33855)
}
