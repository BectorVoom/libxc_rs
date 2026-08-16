//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2079;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta607(t1011: f64, t3120: f64, t23384: f64, t23650: f64, t10336: f64, t1920: f64, t1949: f64, t23323: f64, t6781: f64, t2966: f64, t6805: f64, t135: f64, t23631: f64, t6688: f64, t23637: f64, t23620: f64, t968: f64, t23617: f64, t6680: f64, t10454: f64, t6765: f64, t10889: f64, t3033: f64, t6753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82754, t82789, t82799, t82806, t82809, t82822) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2079(t1011, t3120, t23384, t23650, t10336, t1920, t1949, t23323, t6781, t2966, t6805, t135, t23631, t6688);
        let (t82823, t82828, t82830, t82843, t82848) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2080(t23637, t82822, t1920, t23620, t968, t23617, t6680, t10454, t6765, t10889, t3033, t6753);
    (t82754, t82789, t82799, t82806, t82809, t82822, t82823, t82828, t82830, t82843, t82848)
}
