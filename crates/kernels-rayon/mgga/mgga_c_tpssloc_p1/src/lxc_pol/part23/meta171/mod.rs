//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta171(t1891: f64, t67: f64, t246: f64, t2628: f64, t835: f64, t812: f64, t2690: f64, t815: f64, t116: f64, t126: f64, t136: f64, t16: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9645, t9646, t9666, t9667, t9670, t9671, t9688, t9689) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk784(t1891, t67, t246, t2628, t835, t812, t2690, t815, t116, t126, t136, t16);
    (t9645, t9646, t9666, t9667, t9670, t9671, t9688, t9689)
}
