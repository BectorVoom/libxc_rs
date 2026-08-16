//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 643/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk643(t5038: f64, t657: f64, t1699: f64, t395: f64, t191: f64, t784: f64, t190: f64, t212: f64, t4373: f64, t626: f64, t1251: f64, t658: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5039 = t657 * t5038;
    let t5042 = t395 * t1699;
    let t5044 = t784 * t191;
    let t5047 = 0.29629629629629629629e-1_f64 * t190 * t5044 * t212;
    let t5048 = t626 * t4373;
    let t5049 = t657 * t5048;
    let t5052 = t1251 * t658;
    (t5039, t5042, t5044, t5047, t5048, t5049, t5052)
}
