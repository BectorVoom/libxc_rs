//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 764/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk764(t3085: f64, t594: f64, t4389: f64, t899: f64, t1415: f64, t1397: f64, t9297: f64, t9290: f64, t1457: f64, t9424: f64, t4779: f64, t584: f64, t9419: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30795 = t594 * t3085;
    let t30829 = t4389 * t899;
    let t30830 = t1415 * t30829;
    let t30839 = t1397 * t9297;
    let t30845 = t1415 * t9290;
    let t30936 = t1457 * t9424;
    let t31037 = t584 * t4779 * t9419;
    (t30795, t30829, t30830, t30839, t30845, t30936, t31037)
}
