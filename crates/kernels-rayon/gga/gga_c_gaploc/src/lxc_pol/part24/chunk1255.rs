//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1255/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1255(t23575: f64, t2972: f64, t10805: f64, t5552: f64, t1960: f64, t2728: f64, t3073: f64, t7822: f64, t7332: f64, t8862: f64, t11125: f64, t11127: f64, t1955: f64, t31478: f64, t31480: f64, t31483: f64, t31485: f64, t32090: f64, t32091: f64, t32093: f64, t32095: f64, t32099: f64, t3511: f64, t5549: f64, t841: f64) -> f64 {
    let t32723 = 4.0_f64 * t23575 * t2972;
    let t32731 = 4.0_f64 * t5552 * t10805;
    let t32734 = 4.0_f64 * t1960 * t3073 * t2728;
    let t32736 = 2.0_f64 * t7822 * t3073;
    let t32740 = 2.0_f64 * t8862 * t7332;
    let t32741 = 4.0_f64 * t11125 * t1960 * t841 - 2.0_f64 * t11125 * t1955 + 4.0_f64 * t11127 * t5552 - t3511 * t5549 - t31478 - t31480 - t31483 + t31485 - t32090 + t32091 + t32093 - t32095 - t32099 + t32723 + t32731 + t32734 - t32736 + t32740;
    t32741
}
