//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1019/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1019(t12804: f64, t17268: f64, t587: f64, t12809: f64, t1820: f64, t5125: f64, t12588: f64, t5175: f64, t12575: f64, t1630: f64, t639: f64, t12709: f64, t626: f64) -> (f64, f64, f64, f64, f64) {
    let t41418 = t587 * t17268 * t12804;
    let t41421 = t1820 * t5125 * t12809;
    let t41432 = t5175 * t12588;
    let t41447 = t639 * t1630 * t12575;
    let t41459 = t12709 * t626;
    (t41418, t41421, t41432, t41447, t41459)
}
