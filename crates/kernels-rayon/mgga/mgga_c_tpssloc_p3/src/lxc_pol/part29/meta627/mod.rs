//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2070;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2071;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta627(t11841: f64, t7310: f64, t11791: f64, t7345: f64, t11820: f64, t7339: f64, t11698: f64, t24741: f64, t2132: f64, t24746: f64, t86202: f64, t11754: f64, t86197: f64, t1170: f64, t2121: f64, t24611: f64, t225: f64, t24871: f64, t2122: f64, t7319: f64, t24574: f64, t24597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86343, t86348, t86350, t86354, t86357, t86365) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2070(t11841, t7310, t11791, t7345, t11820, t7339, t11698, t24741, t2132, t24746, t86202, t11754);
        let (t86368, t86390, t86400, t86403, t86409) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2071(t2132, t24746, t86197, t1170, t2121, t24611, t225, t24871, t2122, t7319, t24574, t24597);
    (t86343, t86348, t86350, t86354, t86357, t86365, t86368, t86390, t86400, t86403, t86409)
}
