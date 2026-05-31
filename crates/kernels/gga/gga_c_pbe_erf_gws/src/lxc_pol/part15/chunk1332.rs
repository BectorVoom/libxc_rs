//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1332/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1332<F: Float>(t13796: F, t14724: F, t2352: F, t343: F, t3989: F, t13972: F, t14684: F, t14767: F, t2397: F, t1134: F, t13776: F, t2410: F, t50956: F) -> (F, F, F, F) {
    let t54461 = t3989 * t13796 * t14724 * t343 * t2352;
    let t54463 = t13972 * t14684;
    let t54464 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t54463;
    let t54465 = t14767 * t2397;
    let t54473 = t13776 * t50956 * t1134 * t2410;
    (t54461, t54464, t54465, t54473)
}
