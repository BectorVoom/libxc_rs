//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2474/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2474(t11091: f64, t1637: f64, t43637: f64, t4700: f64, t49082: f64, t49084: f64, t49086: f64, t49088: f64, t49090: f64, t49092: f64, t49095: f64, t49535: f64, t49538: f64, t49540: f64) -> f64 {
    let t50771 = -6.0_f64 * t11091 * t1637 * t43637 * t4700 - t49082 + t49084 - t49086 + t49088 - t49090 + t49092 - t49095 + t49535 + t49538 - t49540;
    t50771
}
