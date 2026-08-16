//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta27 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk197;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk198;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk199;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk200;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk201;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk202;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta27(t28: f64, t517: f64, t148: f64, t516: f64, t157: f64, zeta_threshold: f64, t184: f64, t25: f64, t17: f64, t182: f64, t514: f64, t194: f64, t154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t518, t521) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk197(t28, t517, t148, t516, t157, zeta_threshold);
        let t522 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk198(t184, t521);
        let (t523, t525, t526, t528, t531) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk199(t25, t28, t17, t522, t182, t521, t514, t194, t517, zeta_threshold);
        let t532 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk200(t531);
        let t533 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk201(t531, t532);
        let t534 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk202(t532);
        let t535 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk203(t154, t534);
    (t518, t521, t522, t523, t525, t526, t528, t531, t532, t533, t534, t535)
}
