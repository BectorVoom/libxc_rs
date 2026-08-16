//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 757/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk757(t18540: f64, t201: f64, t1397: f64, t8410: f64, t1359: f64, t3338: f64, t1: f64, t106: f64, t4524: f64, t544: f64, t191: f64, t4529: f64) -> (f64, f64, f64, f64, f64) {
    let t34401 = t201 * t18540;
    let t34471 = t1397 * t8410;
    let t34478 = t1359 * t3338;
    let t34506 = t544 * t4524 * t1 * t106;
    let t34507 = t191 * t4529;
    (t34401, t34471, t34478, t34506, t34507)
}
