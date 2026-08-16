//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2194;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta609(t11677: f64, t11904: f64, t11702: f64, t3536: f64, t11709: f64, t11745: f64, t11651: f64, t11734: f64, t1174: f64, t3556: f64, t698: f64, t11844: f64, t135: f64, t11849: f64, t11153: f64, t1176: f64, t11881: f64, t45113: f64, t11773: f64, t1227: f64, t13969: f64, t11168: f64, t3431: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45162, t45167, t45169, t45171, t45178, t45181) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2194(t11677, t11904, t11702, t3536, t11709, t11745, t11651, t11734, t1174, t3556, t698, t11844, t135);
        let (t45184, t45192, t45197, t45211, t45222) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2195(t1174, t11849, t135, t11153, t1176, t11881, t45113, t11773, t1227, t13969, t11168, t3431);
    (t45162, t45167, t45169, t45171, t45178, t45181, t45184, t45192, t45197, t45211, t45222)
}
