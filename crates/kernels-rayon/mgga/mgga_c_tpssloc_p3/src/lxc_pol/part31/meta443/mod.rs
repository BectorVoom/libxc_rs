//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta443(t1860: f64, t23993: f64, t6509: f64, t7031: f64, t22819: f64, t22825: f64, t22858: f64, t22863: f64, t22867: f64, t22645: f64, t225: f64, t7192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23995, t23998, t23999, t24049, t24050, t24058, t24060, t24061, t24071, t24082) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1590(t1860, t23993, t6509, t7031, t22819, t22825, t22858, t22863, t22867, t22645, t225, t7192);
    (t23995, t23998, t23999, t24049, t24050, t24058, t24060, t24061, t24071, t24082)
}
