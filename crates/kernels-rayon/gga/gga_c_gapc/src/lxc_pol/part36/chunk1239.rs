//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1239/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1239(t3646: f64, t8489: f64, t1464: f64, t3651: f64, t4059: f64, t11248: f64, t1444: f64, t4855: f64, t25042: f64, t4050: f64, t15260: f64, t3948: f64) -> (f64, f64, f64, f64, f64) {
    let t35533 = t8489 * t3646;
    let t35536 = t3651 * t4059 * t1464;
    let t35539 = t11248 * t1444 * t4855;
    let t35541 = t25042 * t4050;
    let t35543 = t35541 * t3948 * t15260;
    (t35533, t35536, t35539, t35541, t35543)
}
