//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1024/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1024(t12634: f64, t5129: f64, t587: f64, t12583: f64, t626: f64, t12766: f64, t1620: f64, t4934: f64, t1033: f64, t10415: f64, t2756: f64, t3488: f64) -> (f64, f64, f64, f64, f64) {
    let t42050 = t587 * t5129 * t12634;
    let t42094 = t12583 * t626;
    let t42109 = t1620 * t4934 * t12766;
    let t42131 = t1033 * t10415;
    let t42142 = t3488 * t2756;
    (t42050, t42094, t42109, t42131, t42142)
}
