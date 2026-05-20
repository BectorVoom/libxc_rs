//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2023;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta600<F: Float>(t2030: F, t47567: F, t26069: F, t94806: F, t26054: F, t9686: F, t25877: F, t94801: F, t1419: F, t786: F, t2023: F, t4075: F, t2453: F, t25949: F, t25946: F, t25939: F, t40270: F, t10073: F, t25920: F, t25938: F, t25898: F, t10115: F, t2024: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94867, t94876, t94884, t94886, t94889, t94890, t94901) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2023::<F>(t2030, t47567, t26069, t94806, t26054, t9686, t25877, t94801, t1419, t786, t2023, t4075);
        let (t94914, t94917, t94919, t94921, t94931) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2024::<F>(t2453, t25949, t25946, t25939, t40270, t10073, t25920, t25938, t25898, t94889, t10115, t2024);
    (t94867, t94876, t94884, t94886, t94890, t94901, t94914, t94917, t94919, t94921, t94931)
}
