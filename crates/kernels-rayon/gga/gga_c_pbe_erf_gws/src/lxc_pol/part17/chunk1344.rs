//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1344/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1344(t1178: f64, t371: f64, t3983: f64, t9258: f64, t1193: f64, t13888: f64, t14747: f64, t2408: f64, t2409: f64, t3066: f64, t3200: f64, t335: f64, t338: f64, t4053: f64, t51978: f64, t51979: f64, t51992: f64, t54621: f64, t54624: f64, t54627: f64, t54629: f64, t54636: f64, t54639: f64, t54641: f64, t8734: f64, t8939: f64, t9241: f64, t9283: f64, t9326: f64) -> f64 {
    let t54649 = t3983 * t371 * t1178 * t9258;
    let t54660 = -35.0_f64 / 432.0_f64 * t54621 - t54624 / 48.0_f64 - t54627 / 48.0_f64 - t54629 / 24.0_f64 - t51978 + t3066 * t2409 * t8734 * t14747 / 24.0_f64 - t54636 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t51979 - 35.0_f64 / 432.0_f64 * t54639 + 35.0_f64 / 432.0_f64 * t54641 - t335 * t338 * t3200 * t4053 / 48.0_f64 - t54649 / 768.0_f64 - 7.0_f64 / 288.0_f64 * t51992 - t2408 * t9283 * t13888 * t9326 / 24.0_f64 + t9241 * t9283 * t1193 * t8939 / 4.0_f64;
    t54660
}
