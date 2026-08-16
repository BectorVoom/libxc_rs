//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 923/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk923(t4981: f64, t586: f64, t593: f64, t5357: f64, t579: f64, t5372: f64, t645: f64, t1651: f64, t5509: f64, t587: f64, t1648: f64, t5413: f64) -> (f64, f64, f64, f64, f64) {
    let t17298 = t4981 * t586;
    let t17300 = 16.0_f64 / 45.0_f64 * t17298 * t593;
    let t17301 = t579 * t5357;
    let t17302 = 64.0_f64 / 405.0_f64 * t17301;
    let t17303 = t5372 * t586;
    let t17305 = 16.0_f64 / 45.0_f64 * t17303 * t645;
    let t17307 = t587 * t1651 * t5509;
    let t17308 = 64.0_f64 / 45.0_f64 * t17307;
    let t17309 = t1648 * t5413;
    (t17300, t17302, t17305, t17308, t17309)
}
