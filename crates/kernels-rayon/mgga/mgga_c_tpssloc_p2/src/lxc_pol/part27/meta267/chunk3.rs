//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1283/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1283(t1611: f64, t1941: f64, t1607: f64, t1618: f64, t1622: f64, t1935: f64, t1937: f64, t378: f64, t6716: f64, t6717: f64, t6728: f64, t6742: f64, t6755: f64, t6763: f64, t6765: f64, t7574: f64, t7578: f64, t7583: f64) -> (f64, f64) {
    let t7586 = t1611 * t1941;
    let t7593 = t6716 + t6717 * t1607 / 288.0_f64 + t6728 + 0.10093189023535097714e-3_f64 * t7574 * t1937 - 0.10093189023535097714e-3_f64 * t1935 * t7578 + 0.10093189023535097714e-3_f64 * t6742 * t7583 + t7586 * t378 / 1536.0_f64 + t6755 * t1618 / 1536.0_f64 + t6763 + t6765 * t1622 / 2304.0_f64;
    (t7586, t7593)
}
