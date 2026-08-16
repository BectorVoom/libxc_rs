//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1606;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta292(t10294: f64, t10544: f64, t2840: f64, t891: f64, t275: f64, t2843: f64, t290: f64, t2924: f64, t2932: f64, t2860: f64, t919: f64, t2904: f64, t938: f64, t10629: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10675, t10676, t10701, t10702) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1606(t10294, t10544, t2840, t891, t275);
        let (t10704, t10723, t10740, t10747, t10756) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1607(t2843, t290, t2924, t2932, t2860, t919, t2904, t938, t10629, t315);
    (t10675, t10676, t10701, t10702, t10704, t10723, t10740, t10747, t10756)
}
