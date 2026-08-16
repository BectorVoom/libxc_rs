//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1333/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1333(t14473: f64, t840: f64, t14579: f64, t3959: f64, t8756: f64, t14576: f64, t2376: f64, t829: f64, t830: f64, t13972: f64, t14608: f64, t1193: f64, t2410: f64, t3207: f64, t36200: f64, t36201: f64, t4155: f64, t50919: f64, t50924: f64, t51906: f64, t54461: f64, t54464: f64, t54465: f64, t54473: f64, t827: f64, t8629: f64, t8759: f64, t8793: f64, t8804: f64, t9283: f64) -> f64 {
    let t54480 = 7.0_f64 / 144.0_f64 * t840 * t14473;
    let t54482 = 7.0_f64 / 144.0_f64 * t840 * t14579;
    let t54484 = t3959 * t8756;
    let t54486 = t2376 * t14576;
    let t54488 = t829 * t830 * t54486;
    let t54491 = t13972 * t14608;
    let t54492 = 7.0_f64 / 2304.0_f64 * t54491;
    let t54493 = -t3207 * t9283 * t1193 * t8804 / 8.0_f64 - t3207 * t9283 * t1193 * t8759 / 16.0_f64 + t54461 / 3072.0_f64 - t54464 + t54465 / 48.0_f64 + t36200 * t36201 * t4155 * t2410 / 4.0_f64 - t54473 / 384.0_f64 - t8793 * t50919 / 12.0_f64 - t8629 * t50924 / 24.0_f64 + t54480 + t54482 + 7.0_f64 / 288.0_f64 * t51906 + t54484 / 24.0_f64 - t827 * t54488 / 48.0_f64 + t54492;
    t54493
}
