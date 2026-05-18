//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1119/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1119<F: Float>(t1208: F, t2242: F, t4090: F, t4414: F, t1205: F, t6781: F, t829: F, t830: F) -> (F, F, F) {
    let t14302 = F::new(35.0) / F::new(432.0) * t2242 * t1208;
    let t14305 = t4414 * t4090;
    let t14309 = t6781 * t1205;
    let t14311 = t829 * t830 * t14309;
    (t14302, t14305, t14311)
}
