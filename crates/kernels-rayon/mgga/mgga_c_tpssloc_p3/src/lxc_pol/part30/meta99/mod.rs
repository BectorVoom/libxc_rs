//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk640;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk641;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk642;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk643;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk644;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta99(t2225: f64, t594: f64, t598: f64, t15: f64, t19: f64, t601: f64, t604: f64, t84: f64, t85: f64, t24: f64, t42: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2226, t2228, t2229) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk640(t2225, t594, t598, t15);
        let t2230 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk641(t2229);
        let (t2232, t2235) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk642(t19, t2230, t601, t604);
        let t2239 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk643(t84, t85);
        let t2240 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk644(t2239, t24);
        let t2267 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk645(t42);
    (t2226, t2228, t2229, t2230, t2232, t2235, t2239, t2240, t2267)
}
