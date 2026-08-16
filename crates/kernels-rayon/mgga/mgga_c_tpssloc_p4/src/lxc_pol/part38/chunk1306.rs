//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1306/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1306(t29934: f64, t510: f64, t574: f64, t1393: f64, t8143: f64, t2180: f64, t3929: f64, t3652: f64, t1268: f64, t12734: f64, t12739: f64, t12823: f64, t2181: f64, t2183: f64, t2314: f64, t29890: f64, t4034: f64, t5113: f64, t652: f64, t8124: f64, t8144: f64, t8148: f64, t8150: f64, t9348: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29935 = t510 * t29934;
    let t29944 = t29934 * t574;
    let t29947 = t8143 * t1393;
    let t29956 = t2180 * t3929;
    let t29963 = t3652 * t2180;
    let t29978 = 2.0_f64 * t1268 * t29944 + 4.0_f64 * t1268 * t29947 + 2.0_f64 * t1268 * t29956 - 4.0_f64 * t12734 * t2181 + 4.0_f64 * t12734 * t2183 + 2.0_f64 * t12739 * t2183 - 2.0_f64 * t12823 * t2181 - 2.0_f64 * t2181 * t9348 + 2.0_f64 * t2183 * t9348 - 4.0_f64 * t2314 * t8124 - 4.0_f64 * t2314 * t8144 + 4.0_f64 * t2314 * t8148 + 4.0_f64 * t2314 * t8150 - 4.0_f64 * t29890 * t652 - 2.0_f64 * t29935 * t652 - 2.0_f64 * t29963 * t652 - 4.0_f64 * t4034 * t8124 - 4.0_f64 * t4034 * t8144 + 4.0_f64 * t5113 * t8148 + 4.0_f64 * t5113 * t8150;
    (t29935, t29944, t29947, t29956, t29963, t29978)
}
