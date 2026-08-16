//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1686/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1686(t1774: f64, t7056: f64, t1266: f64, t7801: f64, t12725: f64, t1442: f64, t1459: f64, t2036: f64, t2040: f64, t2075: f64, t2314: f64, t23938: f64, t27188: f64, t27215: f64, t4026: f64, t4034: f64, t4073: f64, t4077: f64, t5107: f64, t574: f64, t652: f64, t672: f64, t7040: f64, t7042: f64, t7156: f64, t7787: f64, t7802: f64) -> (f64, f64, f64) {
    let t27219 = t1774 * t7056;
    let t27226 = t1266 * t7801;
    let t27238 = -t1266 * t7787 - 2.0_f64 * t12725 * t2040 - t1442 * t7156 - 2.0_f64 * t1459 * t23938 - t1774 * t7040 - t2036 * t5107 - t2075 * t4026 - 2.0_f64 * t2314 * t7802 - 2.0_f64 * t27188 * t672 + t27215 * t574 - 2.0_f64 * t27219 * t652 - 2.0_f64 * t27226 * t652 - 2.0_f64 * t4034 * t7802 - 2.0_f64 * t4073 * t7042 - 2.0_f64 * t4077 * t7042;
    (t27219, t27226, t27238)
}
