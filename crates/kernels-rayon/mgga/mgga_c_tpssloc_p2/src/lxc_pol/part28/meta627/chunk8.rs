//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1963/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1963(t5: f64, t91888: f64, t91914: f64, t91938: f64, t91966: f64, t91993: f64, t92019: f64, t92039: f64, t92068: f64, t112: f64, t111: f64, t26966: f64, t12813: f64, t1458: f64, t2039: f64, t2363: f64, t23917: f64, t23938: f64, t26977: f64, t27188: f64, t4028: f64, t4072: f64, t45632: f64, t55962: f64, t671: f64, t7042: f64, t84097: f64, t90381: f64, t91854: f64, t91857: f64, t91870: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t92072 = piecewise3(t8, 0.0_f64, t91888 + t91914 + t91938 + t91966 + t91993 + t92019 + t92039 + t92068);
    let t92073 = t92072 * t112;
    let t92090 = t26966 * t111;
    let t92099 = 2.0_f64 * t12813 * t7042 + 2.0_f64 * t1458 * t84097 + 4.0_f64 * t1458 * t91854 + 2.0_f64 * t1458 * t91857 + 2.0_f64 * t2039 * t45632 + 2.0_f64 * t2039 * t55962 + 2.0_f64 * t2039 * t90381 + 2.0_f64 * t2363 * t27188 + 2.0_f64 * t23917 * t4028 + 4.0_f64 * t23938 * t4072 + 4.0_f64 * t26977 * t4072 + 4.0_f64 * t671 * t92090 + 2.0_f64 * t91870 + t92073;
    (t92073, t92090, t92099)
}
