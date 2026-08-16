//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1328/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1328(t12823: f64, t1774: f64, t19456: f64, t2181: f64, t2183: f64, t2314: f64, t26114: f64, t26179: f64, t29890: f64, t29934: f64, t30181: f64, t30186: f64, t30189: f64, t30201: f64, t30203: f64, t30209: f64, t30211: f64, t4028: f64, t4034: f64, t45632: f64, t5113: f64, t55962: f64, t652: f64, t8124: f64, t8148: f64, t8221: f64, t8231: f64, t8235: f64, t8237: f64, t90370: f64, t91753: f64, t9348: f64) -> f64 {
    let t110778 = 4.0_f64 * t2314 * t30181 - 2.0_f64 * t12823 * t8221 - 4.0_f64 * t4034 * t30209 + 4.0_f64 * t90370 * t2183 + 4.0_f64 * t26114 * t8148 + 4.0_f64 * t5113 * t30211 - 2.0_f64 * t55962 * t2181 - 4.0_f64 * t19456 * t8124 + 2.0_f64 * t45632 * t2183 - 4.0_f64 * t2314 * t30189 + 4.0_f64 * t2314 * t30201 + 4.0_f64 * t5113 * t30186 + 2.0_f64 * t9348 * t8235 + 2.0_f64 * t9348 * t8237 - 2.0_f64 * t91753 * t2181 - 4.0_f64 * t26179 * t8124 - 2.0_f64 * t652 * t1774 * t29934 - 2.0_f64 * t9348 * t8231 - 4.0_f64 * t4028 * t29890 - 4.0_f64 * t2314 * t30203;
    t110778
}
