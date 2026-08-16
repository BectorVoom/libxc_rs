//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1963/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1963<F: Float>(t5: F, t91888: F, t91914: F, t91938: F, t91966: F, t91993: F, t92019: F, t92039: F, t92068: F, t112: F, t111: F, t26966: F, t12813: F, t1458: F, t2039: F, t2363: F, t23917: F, t23938: F, t26977: F, t27188: F, t4028: F, t4072: F, t45632: F, t55962: F, t671: F, t7042: F, t84097: F, t90381: F, t91854: F, t91857: F, t91870: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t92072 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t91888 + t91914 + t91938 + t91966 + t91993 + t92019 + t92039 + t92068);
    let t92073 = t92072 * t112;
    let t92090 = t26966 * t111;
    let t92099 = F::cast_from(2.0_f64) * t12813 * t7042 + F::cast_from(2.0_f64) * t1458 * t84097 + F::cast_from(4.0_f64) * t1458 * t91854 + F::cast_from(2.0_f64) * t1458 * t91857 + F::cast_from(2.0_f64) * t2039 * t45632 + F::cast_from(2.0_f64) * t2039 * t55962 + F::cast_from(2.0_f64) * t2039 * t90381 + F::cast_from(2.0_f64) * t2363 * t27188 + F::cast_from(2.0_f64) * t23917 * t4028 + F::cast_from(4.0_f64) * t23938 * t4072 + F::cast_from(4.0_f64) * t26977 * t4072 + F::cast_from(4.0_f64) * t671 * t92090 + F::cast_from(2.0_f64) * t91870 + t92073;
    (t92073, t92090, t92099)
}
