//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 836/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk836(t1069: f64, t2795: f64, t3375: f64, t9903: f64, t3367: f64, t3374: f64, t3371: f64, t2405: f64, t2636: f64, t3378: f64, t8666: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9995 = t1069 * t2795;
    let t9997 = t9903 * t3375;
    let t9999 = t3367 * t3374;
    let t10000 = t3371 * t9999;
    let t10002 = t2636 * t2405;
    let t10003 = t3378 * t10002;
    let t10005 = t916 * t8666;
    (t9995, t9997, t9999, t10000, t10003, t10005)
}
