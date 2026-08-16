//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta642(t90472: f64, t1799: f64, t3886: f64, t1887: f64, t80827: f64, t26334: f64, t26339: f64, t81159: f64, t22716: f64, t7697: f64, t26216: f64, t26210: f64, t6897: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90473, t90488, t90497, t90498, t90501, t90503, t90512, t90514) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2060(t90472, t1799, t3886, t1887, t80827, t26334, t26339, t81159, t22716, t7697, t26216, t26210, t6897, t794);
    (t90473, t90488, t90497, t90498, t90501, t90503, t90512, t90514)
}
