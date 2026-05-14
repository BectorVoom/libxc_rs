//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1035/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1035<F: Float>(t2409: F, t4088: F, t8589: F, t14129: F, t14131: F, t14182: F, t14193: F, t14800: F, t14806: F, t14812: F, t15018: F, t15022: F, t15027: F, t15036: F, t15084: F, t2408: F, t3066: F, t335: F, t6793: F, t8629: F, t8793: F) -> (F, F) {
    let t15089 = t2409 * t8589 * t4088;
    let t15094 = -t14129 - t335 * t15018 / 96.0 - t2408 * t15022 / 24.0 + t3066 * t15027 / 48.0 + t8793 * t14182 / 48.0 + t8629 * t14193 / 96.0 + t6793 * t15036 / 48.0 - t335 * t15084 / 96.0 - t14131 + t14800 / 768.0 + t2408 * t15089 / 48.0 + t14806 / 24.0 + t14812 / 24.0;
    (t15089, t15094)
}
