//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1095/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1095(t47067: f64, t47072: f64, t42498: f64, t12032: f64, t2497: f64, t12148: f64, t1382: f64, t921: f64, t13838: f64, t5559: f64, t841: f64, t12270: f64, t1960: f64, t977: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47073 = t47067 + t47072;
    let t47074 = 6.0_f64 * t42498;
    let t47075 = t12032 * t2497;
    let t47077 = t1382 * t12148 * t921;
    let t47078 = 2.0_f64 * t47077;
    let t47080 = t5559 * t13838 * t841;
    let t47083 = t1960 * t12270 * t977;
    (t47073, t47074, t47075, t47078, t47080, t47083)
}
