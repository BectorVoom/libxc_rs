//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1610;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1611;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta294(t2887: f64, t310: f64, t2791: f64, t888: f64, t2897: f64, t942: f64, t2929: f64, t938: f64, t10523: f64, t315: f64, t10544: f64, t1004: f64, t3047: f64, t3053: f64, t3117: f64, t1043: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10813, t10817, t10820, t10825, t10828) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1610(t2887, t310, t2791, t888, t2897, t942, t2929, t938, t10523, t315);
        let (t10832, t10863) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1611(t10544, t1004, t3047);
        let (t10866, t10868) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1612(t3053, t3117, t1043, t676);
    (t10813, t10817, t10820, t10825, t10828, t10832, t10863, t10866, t10868)
}
