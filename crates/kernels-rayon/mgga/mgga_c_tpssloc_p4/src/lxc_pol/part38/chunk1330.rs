//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1330/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1330(t1268: f64, t12734: f64, t12739: f64, t12823: f64, t1393: f64, t19456: f64, t2181: f64, t2314: f64, t26114: f64, t29935: f64, t29944: f64, t30180: f64, t30181: f64, t30195: f64, t30209: f64, t30211: f64, t3652: f64, t3929: f64, t4034: f64, t45632: f64, t5113: f64, t652: f64, t7458: f64, t7676: f64, t8124: f64, t8150: f64, t8221: f64, t8230: f64, t8231: f64, t8235: f64, t8237: f64, t90370: f64, t9348: f64) -> f64 {
    let t110870 = -2.0_f64 * t652 * t3652 * t8230 + 2.0_f64 * t1268 * t8230 * t3929 + 2.0_f64 * t7676 * t29944 + 4.0_f64 * t19456 * t8150 - 2.0_f64 * t45632 * t2181 - 4.0_f64 * t12734 * t8221 - 4.0_f64 * t2314 * t30209 + 4.0_f64 * t2314 * t30211 + 2.0_f64 * t12739 * t8235 + 4.0_f64 * t5113 * t30181 + 4.0_f64 * t1268 * t30180 * t1393 - 2.0_f64 * t12823 * t8231 - 4.0_f64 * t4034 * t30195 - 2.0_f64 * t7458 * t29935 - 2.0_f64 * t9348 * t8221 - 4.0_f64 * t90370 * t2181 - 4.0_f64 * t26114 * t8124 - 4.0_f64 * t12734 * t8231 - 4.0_f64 * t2314 * t30195 + 2.0_f64 * t12739 * t8237;
    t110870
}
