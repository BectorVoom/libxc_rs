//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta671(t24649: f64, t27710: f64, t23508: f64, t8026: f64, t27628: f64, t7324: f64, t7331: f64, t15730: f64, t7339: f64, t24661: f64, t27491: f64, t24668: f64, t27497: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t95323, t95326, t95332, t95334, t95335, t95340, t95346) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2104(t24649, t27710, t23508, t8026, t27628, t7324, t7331, t15730, t7339, t24661, t27491, t24668, t27497);
    (t95323, t95326, t95332, t95334, t95335, t95340, t95346)
}
