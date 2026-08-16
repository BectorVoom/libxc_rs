//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1834;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta590(t1361: f64, t22690: f64, t22792: f64, t5187: f64, t1307: f64, t7708: f64, t80840: f64, t90787: f64, t26245: f64, t80783: f64, t22897: f64, t6925: f64, t26302: f64, t80958: f64, t22779: f64, t26323: f64, t1336: f64, t242: f64, t80901: f64, t5303: f64, t80820: f64, t80915: f64, t22783: f64, t5310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91327, t91344, t91346, t91351) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1834(t1361, t22690, t22792, t5187, t1307, t7708, t80840, t90787, t26245, t80783, t22897, t6925);
        let (t91356, t91358, t91361, t91364, t91383, t91386) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1835(t26302, t80958, t22779, t26323, t1336, t242, t80901, t5303, t80820, t80915, t22783, t5310);
    (t91327, t91344, t91346, t91351, t91356, t91358, t91361, t91364, t91383, t91386)
}
