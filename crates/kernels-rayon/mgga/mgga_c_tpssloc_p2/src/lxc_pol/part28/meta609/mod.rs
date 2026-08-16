//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1919;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta609(t22704: f64, t22705: f64, t26466: f64, t26461: f64, t26433: f64, t6883: f64, t22716: f64, t7741: f64, t1834: f64, t3791: f64, t1992: f64, t550: f64, t6976: f64, t54840: f64, t54883: f64, t22633: f64, t22897: f64, t26421: f64, t3793: f64, t16041: f64, t5336: f64, t80798: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90859, t90864, t90866, t90868, t90870, t90873) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1919(t22704, t22705, t26466, t26461, t26433, t6883, t22716, t7741, t1834, t3791, t1992, t550, t6976);
        let (t90883, t90887, t90892, t90895, t90898) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1920(t1992, t54840, t550, t6976, t54883, t22633, t22897, t26421, t3793, t16041, t22704, t5336, t80798);
    (t90859, t90864, t90866, t90868, t90870, t90873, t90883, t90887, t90892, t90895, t90898)
}
