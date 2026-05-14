//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1142/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1142<F: Float>(t54152: F, t14007: F, t9438: F, t51252: F, t54133: F, t54136: F, t54137: F, t54139: F, t54142: F, t54144: F, t54146: F, t54148: F, t54150: F, t14069: F, t9111: F, t14064: F, t3108: F) -> (F, F, F) {
    let t54153 = 7.0 / 144.0 * t54152;
    let t54154 = t14007 * t9438;
    let t54156 = t54133 / 16.0 - t54136 + t54137 / 256.0 + 3.0 / 256.0 * t54139 - 7.0 / 288.0 * t51252 + t54142 / 96.0 - t54144 / 384.0 - t54146 / 96.0 + t54148 / 48.0 - t54150 / 96.0 + t54153 - t54154 / 384.0;
    let t54158 = t9111 * t14069;
    let t54160 = t3108 * t14064;
    (t54156, t54158, t54160)
}
