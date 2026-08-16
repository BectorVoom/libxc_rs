//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2095;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta668(t91310: f64, t26245: f64, t80791: f64, t26271: f64, t80836: f64, t1361: f64, t22690: f64, t22792: f64, t5187: f64, t1307: f64, t7708: f64, t80840: f64, t90787: f64, t80783: f64, t22897: f64, t6925: f64, t26302: f64, t80958: f64, t22779: f64, t26323: f64, t1336: f64, t242: f64, t80901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91311, t91312, t91323, t91328, t91344) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2095(t91310, t26245, t80791, t26271, t80836, t1361, t22690, t22792, t5187, t1307, t7708, t80840, t90787);
        let (t91345, t91346, t91351, t91357, t91359, t91361) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2096(t91344, t26245, t80783, t22897, t6925, t26302, t80958, t22779, t26323, t1336, t242, t80901);
    (t91311, t91312, t91323, t91328, t91345, t91346, t91351, t91357, t91359, t91361)
}
