//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 534/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk534(t2859: f64, t9333: f64, t3410: f64, t4614: f64, t1562: f64, t3411: f64, t4953: f64, t8097: f64, t874: f64, t1445: f64, t1641: f64, t3391: f64) -> (f64, f64, f64, f64, f64) {
    let t10358 = 0.10725146985555128001e1_f64 * t2859 * t9333;
    let t10359 = t4614 * t3410;
    let t10361 = 0.92023022289409799224e1_f64 * t1562 * t10359;
    let t10363 = 0.69017266717057349418e1_f64 * t4953 * t3411;
    let t10364 = t8097 * t874;
    let t10365 = t1445 * t10364;
    let t10367 = 0.69017266717057349418e1_f64 * t1562 * t10365;
    let t10369 = 0.46011511144704899612e1_f64 * t1641 * t3391;
    (t10358, t10361, t10363, t10367, t10369)
}
