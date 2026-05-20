//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta569<F: Float>(t26179: F, t28105: F, t28109: F, t28112: F, t7349: F, t28116: F, t28119: F, t26169: F, t7709: F, t60221: F, t7342: F, t28093: F) -> (F, F, F, F, F, F, F, F) {
        let (t101872, t101874, t101879, t101881, t101883, t101885, t101886, t101899) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1893::<F>(t26179, t28105, t28109, t28112, t7349, t28116, t28119, t26169, t7709, t60221, t7342, t28093);
    (t101872, t101874, t101879, t101881, t101883, t101885, t101886, t101899)
}
