//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 970/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk970<F: Float>(t3065: F, t8605: F, t2501: F, t814: F, t829: F, t830: F, t1114: F, t6111: F, t2367: F, t3052: F, t2395: F, t2409: F, t3189: F) -> (F, F, F, F, F) {
    let t8606 = t3065 * t8605;
    let t8611 = t829 * t830 * t2501 * t814;
    let t8616 = t1114 * t6111;
    let t8622 = F::new(7.0) / F::new(72.0) * t2367 * t3052;
    let t8624 = t2409 * t2395 * t3189;
    (t8606, t8611, t8616, t8622, t8624)
}
