//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1918;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta646(t16662: f64, t6552: f64, t6553: f64, t6554: f64, t23164: f64, t23204: f64, t28276: f64, t16968: f64, t87052: f64, t87053: f64, t16887: f64, t87057: f64, t28342: f64, t81979: f64, t17022: f64, t1880: f64, t1894: f64, t214: f64, t252: f64, t5527: f64, t25038: f64, t6646: f64, t829: f64, t28333: f64, t6562: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98319, t98322, t98325, t98328) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1918(t16662, t6552, t6553, t6554, t23164, t23204, t28276, t16968, t87052, t87053, t16887, t87057);
        let (t98330, t98334, t98336, t98339, t98342) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1919(t28342, t81979, t17022, t1880, t1894, t214, t252, t5527, t25038, t6646, t829, t28333, t6562, t794);
    (t98319, t98322, t98325, t98328, t98330, t98334, t98336, t98339, t98342)
}
