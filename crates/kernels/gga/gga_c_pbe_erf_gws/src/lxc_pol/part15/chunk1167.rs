//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1167/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1167<F: Float>(t54566: F, t3959: F, t9328: F, t2409: F, t26655: F, t3965: F, t14765: F, t3074: F, t4395: F, t2362: F, t1144: F, t13930: F, t14107: F, t29775: F, t335: F, t338: F, t4002: F, t51592: F, t51599: F, t51604: F, t54541: F, t54545: F, t54550: F, t54561: F, t54564: F, t6793: F, t8616: F, t8793: F) -> (F,) {
    let t54567 = 7.0 / 72.0 * t54566;
    let t54572 = t3959 * t9328;
    let t54575 = t3965 * t2409 * t26655;
    let t54580 = t3074 * t4395 * t14765;
    let t54581 = t54580 * t2362;
    let t54583 = t54541 / 1536.0 + t6793 * t54545 / 24.0 + t6793 * t54550 / 24.0 + t29775 * t13930 / 24.0 + t8793 * t51592 / 24.0 + t8793 * t51599 / 24.0 + t8793 * t51604 / 48.0 + t54561 / 96.0 - t54564 / 96.0 + t54567 - t335 * t338 * t1144 * t14107 / 96.0 + t54572 / 48.0 - t54575 / 48.0 - t8616 * t4002 / 96.0 - t54581 / 32.0;
    (t54583,)
}
