//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2003;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta583<F: Float>(t25516: F, t3278: F, t11223: F, t1976: F, t27639: F, t995: F, t19482: F, t988: F, t25610: F, t25604: F, t7156: F, t3268: F, t7143: F, t3057: F, t25460: F, t25698: F, t1071: F, t7150: F, t8521: F, t359: F, t42066: F, t3143: F, t36870: F, t1983: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93821, t93884, t93890, t93893, t93897, t93904, t93920) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2003::<F>(t25516, t3278, t11223, t1976, t27639, t995, t19482, t988, t25610, t25604, t7156, t3268, t7143);
        let (t93921, t93928, t93963, t93968, t93983) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2004::<F>(t3057, t93920, t25460, t25698, t1071, t7150, t8521, t359, t42066, t3143, t36870, t1983);
    (t93821, t93884, t93890, t93893, t93897, t93904, t93920, t93921, t93928, t93963, t93968, t93983)
}
