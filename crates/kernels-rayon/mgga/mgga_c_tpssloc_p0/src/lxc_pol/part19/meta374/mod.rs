//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1391;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1392;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta374(t3247: f64, t39103: f64, t1113: f64, t136: f64, t11545: f64, t241: f64, t3241: f64, t39097: f64, t11229: f64, t699: f64, t11232: f64, t11219: f64, t43732: f64, t242: f64, t281: f64, t415: f64, t2394: f64, t3253: f64, t3249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43757, t43759, t43763, t43764, t43766, t43768, t43770, t43773) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1391(t3247, t39103, t1113, t136, t11545, t241, t3241, t39097, t11229, t699, t11232, t11219, t43732);
        let (t43776, t43777, t43780) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1392(t242, t281, t415, t2394, t3253);
        let t43782 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1393(t2394, t3249);
    (t43757, t43759, t43763, t43764, t43766, t43768, t43770, t43773, t43776, t43777, t43780, t43782)
}
