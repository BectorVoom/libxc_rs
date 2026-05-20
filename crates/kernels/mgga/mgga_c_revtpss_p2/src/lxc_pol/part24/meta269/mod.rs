//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta269<F: Float>(t212: F, t6041: F, t780: F, t689: F, t2703: F, t5985: F, t10905: F, t5989: F, t5962: F, t854: F, t236: F, t807: F) -> (F, F, F, F, F, F, F, F) {
        let (t18316, t18317, t18318, t18338, t18340, t18348, t18349, t18350) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1041::<F>(t212, t6041, t780, t689, t2703, t5985, t10905, t5989, t5962, t854, t236, t807);
    (t18316, t18317, t18318, t18338, t18340, t18348, t18349, t18350)
}
