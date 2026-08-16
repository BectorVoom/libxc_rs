//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 660/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk660(t339: f64, t911: f64, t824: f64, t56: f64, t931: f64, t19: f64, t2132: f64, t328: f64) -> (f64, f64, f64, f64, f64) {
    let t6643 = t339 * t911;
    let t6644 = t824 * t6643;
    let t6658 = t56 * t931;
    let t6659 = t6658 * t19;
    let t6670 = t2132 * t328;
    (t6643, t6644, t6658, t6659, t6670)
}
