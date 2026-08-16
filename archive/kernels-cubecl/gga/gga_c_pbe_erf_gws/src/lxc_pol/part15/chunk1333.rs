//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1333/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1333<F: Float>(t14473: F, t840: F, t14579: F, t3959: F, t8756: F, t14576: F, t2376: F, t829: F, t830: F, t13972: F, t14608: F, t1193: F, t2410: F, t3207: F, t36200: F, t36201: F, t4155: F, t50919: F, t50924: F, t51906: F, t54461: F, t54464: F, t54465: F, t54473: F, t827: F, t8629: F, t8759: F, t8793: F, t8804: F, t9283: F) -> F {
    let t54480 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t14473;
    let t54482 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t14579;
    let t54484 = t3959 * t8756;
    let t54486 = t2376 * t14576;
    let t54488 = t829 * t830 * t54486;
    let t54491 = t13972 * t14608;
    let t54492 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t54491;
    let t54493 = -t3207 * t9283 * t1193 * t8804 / F::cast_from(8.0_f64) - t3207 * t9283 * t1193 * t8759 / F::cast_from(16.0_f64) + t54461 / F::cast_from(3072.0_f64) - t54464 + t54465 / F::cast_from(48.0_f64) + t36200 * t36201 * t4155 * t2410 / F::cast_from(4.0_f64) - t54473 / F::cast_from(384.0_f64) - t8793 * t50919 / F::cast_from(12.0_f64) - t8629 * t50924 / F::cast_from(24.0_f64) + t54480 + t54482 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51906 + t54484 / F::cast_from(24.0_f64) - t827 * t54488 / F::cast_from(48.0_f64) + t54492;
    t54493
}
