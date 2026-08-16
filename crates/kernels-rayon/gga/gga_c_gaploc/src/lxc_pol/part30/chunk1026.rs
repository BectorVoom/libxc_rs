//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1026/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1026(t2972: f64, t7324: f64, t3459: f64, t5552: f64, t3073: f64, t977: f64, t1960: f64, t2595: f64, t8862: f64, t2592: f64, t10282: f64, t10285: f64, t10288: f64, t10289: f64, t10297: f64, t10300: f64, t10625: f64, t11125: f64, t11127: f64, t11130: f64, t748: f64) -> (f64, f64) {
    let t11132 = 2.0_f64 * t7324 * t2972;
    let t11134 = 2.0_f64 * t5552 * t3459;
    let t11135 = t3073 * t977;
    let t11137 = 2.0_f64 * t1960 * t11135;
    let t11139 = 2.0_f64 * t8862 * t2595;
    let t11140 = t2592 * t3073;
    let t11141 = -t11125 * t748 + 2.0_f64 * t11127 * t1960 - t10282 + t10285 + t10288 + t10289 + t10297 - t10300 + t10625 - t11130 + t11132 + t11134 + t11137 + t11139 - t11140;
    (t11135, t11141)
}
