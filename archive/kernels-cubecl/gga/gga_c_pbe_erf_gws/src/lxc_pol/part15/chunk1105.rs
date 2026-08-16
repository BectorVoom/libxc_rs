//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1105/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1105<F: Float>(t14015: F, t2315: F, t2115: F, t4023: F, t2129: F, t345: F, t837: F, t56: F, t859: F) -> (F, F, F, F, F, F) {
    let t14016 = t14015 * t2315;
    let t14018 = t2115 * t4023;
    let t14020 = t2129 * t4023;
    let t14022 = t345 * t837;
    let t14023 = t14022 * t56;
    let t14024 = t14023 * t859;
    (t14016, t14018, t14020, t14022, t14023, t14024)
}
