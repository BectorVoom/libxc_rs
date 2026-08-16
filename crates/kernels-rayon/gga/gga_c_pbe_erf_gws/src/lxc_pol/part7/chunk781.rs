//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 781/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk781(t339: f64, t4379: f64, t2178: f64, t2181: f64, t2183: f64, t2186: f64, t340: f64, t6084: f64, t6421: f64, t6424: f64, t6429: f64, t6430: f64, t6433: f64, t870: f64, t871: f64) -> (f64, f64) {
    let t6436 = t339 * t4379;
    let t6439 = -t339 * t340 * t6084 + 9.0_f64 * t2178 * t2186 - 36.0_f64 * t2181 * t6433 - 36.0_f64 * t2183 * t6424 + 9.0_f64 * t6421 * t871 + 60.0_f64 * t6429 * t6430 + 3.0_f64 * t6436 * t870;
    (t6436, t6439)
}
