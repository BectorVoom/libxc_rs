//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 724/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk724(t1709: f64, t3431: f64, t1174: f64, t3439: f64, t60: f64, t461: f64, t4724: f64, t1409: f64, t3450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4896 = t3431 * t1709;
    let t4897 = t1174 * t4896;
    let t4899 = t60 * t3439;
    let t4900 = t4899 * t461;
    let t4901 = t4900 * t4724;
    let t4904 = t3450 * t1409;
    (t4896, t4897, t4899, t4900, t4901, t4904)
}
