//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2075;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta657(t90470: f64, t26356: f64, t6914: f64, t1799: f64, t3886: f64, t1887: f64, t80827: f64, t26334: f64, t26339: f64, t81159: f64, t22716: f64, t7697: f64, t26216: f64, t26210: f64, t6897: f64, t794: f64, t1377: f64, t5187: f64, t7692: f64, t81186: f64, t26338: f64, t81228: f64, t81326: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90471, t90473, t90488, t90497, t90498, t90501, t90503) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2075(t90470, t26356, t6914, t1799, t3886, t1887, t80827, t26334, t26339, t81159, t22716, t7697);
        let (t90512, t90515, t90516, t90521, t90524) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2076(t26216, t81159, t26210, t6897, t794, t1377, t5187, t7692, t81186, t26338, t81228, t81326);
    (t90471, t90473, t90488, t90497, t90498, t90501, t90503, t90512, t90515, t90516, t90521, t90524)
}
