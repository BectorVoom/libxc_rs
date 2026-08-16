//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 880/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk880(t1393: f64, t2180: f64, t1268: f64, t2181: f64, t2183: f64, t2314: f64, t4034: f64, t5113: f64, t652: f64, t8124: f64, t8144: f64, t8148: f64) -> (f64, f64) {
    let t8150 = t2180 * t1393;
    let t8153 = 2.0_f64 * t1268 * t8148 + 2.0_f64 * t1268 * t8150 - 2.0_f64 * t2181 * t2314 - 2.0_f64 * t2181 * t4034 + 2.0_f64 * t2183 * t2314 + 2.0_f64 * t2183 * t5113 - 2.0_f64 * t652 * t8124 - 2.0_f64 * t652 * t8144;
    (t8150, t8153)
}
