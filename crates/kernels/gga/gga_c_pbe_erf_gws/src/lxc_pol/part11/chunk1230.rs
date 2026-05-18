//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1230/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1230<F: Float>(t49416: F, t49417: F, t49419: F, t49420: F, t49424: F, t49427: F, t49429: F, t49433: F, t3776: F, t3373: F, t1076: F, t11318: F, t12381: F, t13164: F, t13167: F, t2107: F, t21091: F, t22688: F, t3030: F, t323: F, t35109: F, t44405: F, t48520: F, t6096: F, t818: F, t9150: F) -> (F, F) {
    let t49436 = t49416 + t49417 + t49419 + t49420 + t49424 + t49427 + t49429 + t49433;
    let t49450 = t3776 * t3776;
    let t49456 = t3373 * t3373;
    let t49463 = F::new(8.0) * t1076 * t12381 * t2107 - F::new(36.0) * t3373 * t3776 * t6096 - F::new(4.0) * t1076 * t44405 - F::new(6.0) * t11318 * t3373 - F::new(4.0) * t12381 * t3030 - F::new(24.0) * t13164 * t22688 + F::new(24.0) * t13167 * t9150 + F::new(6.0) * t2107 * t49456 + F::new(24.0) * t21091 * t49450 + t323 * t49436 + F::new(12.0) * t35109 * t3776 - t48520 * t818;
    (t49436, t49463)
}
