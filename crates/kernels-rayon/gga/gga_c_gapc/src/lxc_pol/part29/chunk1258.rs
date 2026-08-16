//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1258/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1258(t11228: f64, t25202: f64, t13675: f64, t190: f64, t25813: f64, t35455: f64, t21369: f64, t2936: f64, t11234: f64, t11235: f64, t15284: f64, t4296: f64, t674: f64) -> (f64, f64, f64, f64, f64) {
    let t35506 = t11228 * t25202;
    let t35510 = t35455 * t13675 * t190 * t25813;
    let t35512 = t2936 * t21369;
    let t35515 = t11234 * t11235 * t15284;
    let t35517 = t4296 * t674;
    (t35506, t35510, t35512, t35515, t35517)
}
