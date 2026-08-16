//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1353/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1353(t10541: f64, t8601: f64, t1049: f64, t30867: f64, t11046: f64, t9378: f64, t15436: f64, t3832: f64, t1616: f64, t3179: f64, t3537: f64, t12055: f64, t4908: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36303 = 4.0_f64 * t8601 * t10541;
    let t36304 = t30867 * t1049;
    let t36307 = 4.0_f64 * t11046 * t9378;
    let t36309 = 2.0_f64 * t15436 * t3832;
    let t36312 = 4.0_f64 * t1616 * t3537 * t3179;
    let t36314 = 4.0_f64 * t4908 * t12055;
    (t36303, t36304, t36307, t36309, t36312, t36314)
}
