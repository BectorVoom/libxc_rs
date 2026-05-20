//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2521;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta615<F: Float>(t19666: F, t4806: F, t1042: F, t16208: F, t19661: F, t1065: F, t6258: F, t906: F, t5825: F, t606: F, t4801: F, t1063: F, t15668: F, t15675: F, t15707: F, t19651: F, t19659: F, t19663: F, t3127: F, t3169: F, t4837: F, t4875: F, t6302: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2521::<F>(t19666, t4806, t1042, t16208, t19661, t1065, t6258, t906, t5825, t606);
        let (t19681, t19682, t19685) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2522::<F>(t19680, t4801, t1042, t1063, t15668, t15675, t15707, t19651, t19659, t19663, t19668, t19672, t19677, t3127, t3169, t4837, t4875, t6302);
    (t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680, t19681, t19682, t19685)
}
