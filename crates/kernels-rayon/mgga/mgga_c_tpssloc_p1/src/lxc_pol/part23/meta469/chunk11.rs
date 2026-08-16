//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1397/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1397(t1070: f64, t193: f64, t336: f64, t43637: f64, t76668: f64, t76671: f64, t76674: f64, t76675: f64, t76715: f64, t76997: f64, t77001: f64, t77003: f64, t77006: f64, t77009: f64, t77012: f64, t77014: f64, t77016: f64, t77913: f64) -> f64 {
    let t77918 = t76668 - t76671 + t76674 - 6.0_f64 * t193 * t336 * t76675 * t43637 + t193 * t336 * (t76715 + t77913) * t1070 - t76997 + t77001 + t77003 + t77006 + t77009 - t77012 - t77014 - t77016;
    t77918
}
