//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1307/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1307<F: Float>(t51312: F, t9035: F, t14570: F, t6538: F, t3123: F, t51430: F, t14007: F, t9438: F, t51252: F, t54133: F, t54136: F, t54137: F, t54139: F, t54142: F, t54144: F, t54146: F) -> F {
    let t54148 = t9035 * t51312;
    let t54150 = t6538 * t14570;
    let t54152 = t3123 * t51430;
    let t54153 = F::new(7.0) / F::new(144.0) * t54152;
    let t54154 = t14007 * t9438;
    let t54156 = t54133 / F::new(16.0) - t54136 + t54137 / F::new(256.0) + F::new(3.0) / F::new(256.0) * t54139 - F::new(7.0) / F::new(288.0) * t51252 + t54142 / F::new(96.0) - t54144 / F::new(384.0) - t54146 / F::new(96.0) + t54148 / F::new(48.0) - t54150 / F::new(96.0) + t54153 - t54154 / F::new(384.0);
    t54156
}
