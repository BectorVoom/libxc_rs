//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1899;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta461<F: Float>(t19680: F, t4801: F, t1042: F, t1063: F, t15668: F, t15675: F, t15707: F, t19651: F, t19659: F, t19663: F, t19668: F, t19672: F, t19677: F, t3127: F, t3169: F, t4837: F, t4875: F, t6302: F, t4806: F, t5819: F, t999: F, t1032: F, t6235: F, t1040: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19681, t19682, t19685) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1899::<F>(t19680, t4801, t1042, t1063, t15668, t15675, t15707, t19651, t19659, t19663, t19668, t19672, t19677, t3127, t3169, t4837, t4875, t6302);
        let (t19687, t19688, t19691, t19692, t19693, t19696, t19697) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1900::<F>(t19680, t4806, t1042, t5819, t999, t1032, t6235, t1040);
    (t19681, t19682, t19685, t19687, t19688, t19691, t19692, t19693, t19696, t19697)
}
