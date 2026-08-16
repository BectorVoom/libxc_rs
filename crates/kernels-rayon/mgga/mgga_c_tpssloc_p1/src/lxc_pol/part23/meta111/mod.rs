//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta111(t1474: f64, t67: f64, t758: f64, t228: f64, t68: f64, t1484: f64, t845: f64, t1516: f64, t2697: f64, t1520: f64, t225: f64, t2627: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4211, t4212, t4225, t4226, t4253, t4268, t4280) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk588(t1474, t67, t758, t228, t68, t1484, t845, t1516, t2697, t1520, t225, t2627);
    (t4211, t4212, t4225, t4226, t4253, t4268, t4280)
}
