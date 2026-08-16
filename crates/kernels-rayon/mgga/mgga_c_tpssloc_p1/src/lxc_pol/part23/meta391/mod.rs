//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1195;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta391(t19681: f64, t2535: f64, t2371: f64, t19575: f64, t592: f64, t2221: f64, t6328: f64, t2223: f64, t2225: f64, t17: f64, t2516: f64, t6320: f64, t212: f64, t6330: f64, t2586: f64, t40353: f64, t6347: f64, t12225: f64, t40018: f64, t6353: f64, t12189: f64, t6358: f64, t19767: f64, t40409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56104, t56168, t56185, t56390, t56392, t56394, t56398) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1195(t19681, t2535, t2371, t19575, t592, t2221, t6328, t2223, t2225, t17, t2516, t6320);
        let (t56465, t56469, t56484, t56491, t56535) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1196(t212, t6330, t2586, t40353, t6347, t12225, t40018, t6353, t12189, t6358, t19767, t40409);
    (t56104, t56168, t56185, t56390, t56392, t56394, t56398, t56465, t56469, t56484, t56491, t56535)
}
