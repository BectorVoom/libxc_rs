//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1128/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1128<F: Float>(t13917: F, t53156: F, t9333: F, t22336: F, t4002: F, t14667: F, t22263: F, t2409: F, t3066: F, t4385: F, t51569: F, t51815: F, t51825: F, t51827: F, t51829: F, t53915: F, t53925: F, t53930: F, t53936: F, t53939: F, t53943: F, t8734: F, t8793: F) -> (F,) {
    let t53945 = t13917 * t53156 * t9333;
    let t53948 = 7.0 / 144.0 * t22336 * t4002;
    let t53949 = -7.0 / 72.0 * t51815 - t53915 + 35.0 / 108.0 * t51825 + 7.0 / 4608.0 * t51827 + t3066 * t2409 * t8734 * t14667 / 24.0 - 7.0 / 576.0 * t51829 - t53925 / 12.0 - t8793 * t51569 / 16.0 + t53930 / 192.0 - t22263 * t4002 / 48.0 - t53936 / 768.0 + t4385 * t53939 / 96.0 - t53943 + t53945 / 256.0 + t53948;
    (t53949,)
}
