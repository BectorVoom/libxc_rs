//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1099/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1099<F: Float>(t14058: F, t935: F, t2327: F, t4049: F, t4021: F, t885: F, t2149: F) -> (F, F, F, F, F) {
    let t14059 = t14058 * t935;
    let t14060 = F::new(7.0) / F::new(288.0) * t14059;
    let t14061 = t4049 * t2327;
    let t14063 = t4021 * t885;
    let t14064 = t14063 * t2149;
    (t14059, t14060, t14061, t14063, t14064)
}
