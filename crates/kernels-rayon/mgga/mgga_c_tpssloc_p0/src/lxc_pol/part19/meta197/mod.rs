//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk863;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta197(t10250: f64, t4518: f64, t2775: f64, t343: f64, t2244: f64, t2988: f64, t2987: f64, t3014: f64, t2990: f64, t2262: f64, t972: f64, t10186: f64, t10192: f64, t10196: f64, t10200: f64, t10204: f64, t10209: f64, t10219: f64, t10226: f64, t10229: f64, t10233: f64, t10238: f64, t10242: f64, t10246: f64, t2960: f64, t2982: f64, t2986: f64, t2991: f64, t973: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10251, t10254, t10255, t10256, t10259, t10260, t10263) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk863(t10250, t4518, t2775, t343, t2244, t2988, t2987, t3014, t2990, t2262, t972);
        let t10266 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk864(t10186, t10192, t10196, t10200, t10204, t10209, t10219, t10226, t10229, t10233, t10238, t10242, t10246, t10251, t10256, t10260, t10263, t2960, t2982, t2986, t2991, t973, t980);
    (t10251, t10254, t10255, t10256, t10259, t10260, t10263, t10266)
}
