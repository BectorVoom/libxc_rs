//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 688/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk688(t3205: f64, t329: f64, t838: f64, t1164: f64, t2242: f64, t3133: f64, t6183: f64, t3179: f64, t6331: f64, t1133: f64, t2157: f64, t332: f64, t6238: f64, t863: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8801 = t329 * t838 * t3205;
    let t8818 = t2242 * t1164;
    let t8824 = t6183 * t3133;
    let t8833 = t6331 * t3179;
    let t8884 = t1133 * t2157;
    let t8903 = t863 * t6238 * t332;
    (t8801, t8818, t8824, t8833, t8884, t8903)
}
