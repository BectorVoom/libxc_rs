//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta51 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk342;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta51(t340: f64, t984: f64, t343: f64, t974: f64, t346: f64, t964: f64, t971: f64, t973: f64, t980: f64, t381: f64, t221: f64, t967: f64, t339: f64, t883: f64, t976: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t986, t990) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk342(t340, t984, t343, t974, t346, t964, t971, t973, t980);
        let (t991, t995, t997, t998) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk343(t381, t990, t221, t967, t339, t883, t976);
    (t986, t990, t991, t995, t997, t998)
}
