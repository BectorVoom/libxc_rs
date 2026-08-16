//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1166;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1167;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1168;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta243(t1911: f64, t865: f64, t2718: f64, t1906: f64, t6547: f64, t214: f64, t225: f64, t234: f64, t252: f64, t776: f64, t6552: f64, t1905: f64, t794: f64, t6562: f64, t6604: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6632 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1166(t1911, t865, t2718);
        let (t6636, t6637) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1167(t1906, t6547, t214, t225);
        let t6638 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1168(t234, t252);
        let (t6639, t6640, t6641, t6643, t6645, t6646) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1169(t6638, t776, t6637, t6552, t1905, t794, t6562, t6604, t814);
    (t6632, t6636, t6637, t6638, t6639, t6640, t6641, t6643, t6645, t6646)
}
