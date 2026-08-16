//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk767;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta137(t500: f64, t526: f64, t528: f64, t118: f64, t521: f64) -> (f64, f64, f64, f64, f64) {
        let (t3639, t3640) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk767(t500);
        let (t3664, t3672, t3684) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk768(t526, t528, t118, t521);
    (t3639, t3640, t3664, t3672, t3684)
}
