//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk739;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk740;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk741;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk742;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta123(t2289: f64, t2244: f64, t882: f64, t123: f64, t2250: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t2775 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk739(t2289);
        let t2776 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk740(t2244, t2775);
        let (t2777, t2778) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk741(t2776, t882, t123);
        let t2780 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk742(t2250, t883);
        let (t2781, t2782) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk743(t2780, t882, t123);
    (t2775, t2776, t2777, t2778, t2780, t2781, t2782)
}
