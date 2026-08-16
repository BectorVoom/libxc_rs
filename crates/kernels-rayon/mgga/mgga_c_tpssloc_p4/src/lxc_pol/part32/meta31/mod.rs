//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk228;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk229;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk230;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta31(t588: f64, t15: f64, t3: f64, t14: f64, t2: f64, t21: f64, t583: f64, t19: f64, t582: f64, t586: f64, t83: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t589, t590, t591) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk228(t588, t15, t3);
        let t592 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk229(t14, t591);
        let (t593, t594, t596, t597, t598) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk230(t592, t14, t2, t21, t15, t583);
        let (t600, t601, t604) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk231(t19, t598, t582, t586, t589, t593, t596, t83, t85);
    (t589, t590, t591, t592, t593, t594, t596, t597, t598, t600, t601, t604)
}
