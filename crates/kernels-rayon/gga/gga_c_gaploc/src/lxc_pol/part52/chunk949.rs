//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 949/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk949(t47964: f64, t9287: f64, t2365: f64, t38272: f64, t7025: f64, t38770: f64, t901: f64, t38486: f64, t13792: f64, t4379: f64, t12000: f64, t1429: f64, t2366: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47965 = t47964 * t9287;
    let t47968 = t7025 * t2365 * t38272;
    let t47976 = t38770 * t901;
    let t47978 = t38486 * t901;
    let t47980 = t4379 * t13792;
    let t47984 = t1429 * t2365 * t2366 * t12000;
    (t47965, t47968, t47976, t47978, t47980, t47984)
}
