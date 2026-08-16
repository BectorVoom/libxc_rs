//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 674/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk674(t1885: f64, t5394: f64, t587: f64, t1697: f64, t212: f64, t22: f64, t219: f64, t5063: f64, t4367: f64, t639: f64, t1774: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5395 = t1885 * t5394;
    let t5397 = 4.0_f64 / 5.0_f64 * t587 * t5395;
    let t5399 = 1.0_f64 / t212 / t1697;
    let t5400 = t22 * t5399;
    let t5401 = t219 * t5063;
    let t5402 = t5401 * t4367;
    let t5403 = t5400 * t5402;
    let t5405 = 32.0_f64 / 81.0_f64 * t639 * t5403;
    let t5406 = t1774 * t586;
    (t5395, t5397, t5399, t5400, t5401, t5402, t5403, t5405, t5406)
}
