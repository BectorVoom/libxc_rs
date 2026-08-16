//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1131/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1131(t20296: f64, t343: f64, t2121: f64, t337: f64, t2134: f64, t356: f64, t358: f64, t6552: f64, t2252: f64, t6: f64, t6231: f64, t254: f64) -> (f64, f64, f64, f64) {
    let t20297 = t20296 * t343;
    let t20299 = t2121 * t337 * t20297;
    let t20301 = t2134 * t20299 / 24.0_f64;
    let t20303 = t356 * t358 * t6552;
    let t20304 = t20303 * t2252;
    let t20305 = t6 * t6231;
    let t20306 = t254 * t20305;
    (t20301, t20304, t20305, t20306)
}
