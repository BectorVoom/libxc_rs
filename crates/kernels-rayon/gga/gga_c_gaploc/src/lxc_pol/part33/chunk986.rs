//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 986/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk986(t11058: f64, t10627: f64, t5241: f64, t590: f64, t5640: f64, t1890: f64, t1966: f64, t739: f64, t1991: f64, t10019: f64, t2617: f64, t3005: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11059 = 0.38342925953920749676e0_f64 * t11058;
    let t11061 = t5241 * t10627 * t590;
    let t11063 = 0.15337170381568299871e1_f64 * t5640 * t11061;
    let t11065 = t1890 * t10627 * t590;
    let t11067 = 0.25561950635947166451e1_f64 * t1966 * t11065;
    let t11068 = t739 * t10627;
    let t11069 = t11068 * t590;
    let t11071 = 0.1022478025437886658e1_f64 * t1991 * t11069;
    let t11108 = 0.15976219147466979032e-1_f64 * t10019;
    let t11109 = t3005 * t2617;
    (t11059, t11061, t11063, t11065, t11067, t11069, t11071, t11108, t11109)
}
