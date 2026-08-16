//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1934;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta500(t1615: f64, t5914: f64, t1060: f64, t21594: f64, t381: f64, t21390: f64, t11048: f64, t1625: f64, t5872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t21626, t21627, t21634, t21635, t21637) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1934(t1615, t5914, t1060, t21594, t381, t21390);
        let (t21638, t21643) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1935(t11048, t21637, t1625, t5872);
    (t21626, t21627, t21634, t21635, t21637, t21638, t21643)
}
