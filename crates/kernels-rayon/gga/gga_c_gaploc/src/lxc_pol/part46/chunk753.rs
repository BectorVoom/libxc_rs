//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 753/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk753(t2366: f64, t30208: f64, t1359: f64, t3116: f64, t3085: f64, t1: f64, t29882: f64, t544: f64, t1397: f64, t9290: f64, t2321: f64, t28438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30209 = t2366 * t30208;
    let t30301 = t1359 * t3116;
    let t30334 = t1359 * t3085;
    let t30635 = t544 * t29882 * t1;
    let t30639 = t1397 * t9290;
    let t30733 = t28438 * t2321;
    (t30209, t30301, t30334, t30635, t30639, t30733)
}
