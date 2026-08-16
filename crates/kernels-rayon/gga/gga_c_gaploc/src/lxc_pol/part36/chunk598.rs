//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 598/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk598(t3411: f64, t4953: f64, t8097: f64, t874: f64, t1445: f64, t1562: f64, t1641: f64, t3391: f64, t7980: f64, t574: f64, t2293: f64, t2778: f64) -> (f64, f64, f64, f64, f64) {
    let t10363 = 0.69017266717057349418e1_f64 * t4953 * t3411;
    let t10364 = t8097 * t874;
    let t10365 = t1445 * t10364;
    let t10367 = 0.69017266717057349418e1_f64 * t1562 * t10365;
    let t10369 = 0.46011511144704899612e1_f64 * t1641 * t3391;
    let t10370 = t7980 * t874;
    let t10371 = t1445 * t10370;
    let t10373 = 0.46011511144704899612e1_f64 * t574 * t10371;
    let t10374 = t2778 * t2293;
    (t10363, t10367, t10369, t10373, t10374)
}
