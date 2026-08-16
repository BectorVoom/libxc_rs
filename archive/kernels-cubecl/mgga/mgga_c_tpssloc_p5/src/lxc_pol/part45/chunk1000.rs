//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1000/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1000<F: Float>(t23909: F, t8526: F, t23918: F, t1873: F, t24428: F, t652: F, t112528: F, t112535: F, t112537: F, t112542: F, t114573: F, t115195: F, t115208: F, t1976: F, t2039: F, t22483: F, t2314: F, t2364: F, t23829: F, t23933: F, t23941: F, t31532: F, t31726: F, t31734: F, t4034: F, t6517: F, t7042: F, t8529: F, t9348: F) -> F {
    let t115210 = F::cast_from(2.0_f64) * t8526 * t23909;
    let t115212 = F::cast_from(2.0_f64) * t8526 * t23918;
    let t115217 = F::cast_from(2.0_f64) * t652 * t24428 * t1873;
    let t115222 = -F::cast_from(2.0_f64) * t2039 * t23829 * t652 - F::cast_from(2.0_f64) * t1976 * t23941 - F::cast_from(2.0_f64) * t22483 * t7042 - F::cast_from(4.0_f64) * t2314 * t31726 - F::cast_from(2.0_f64) * t2364 * t31532 - F::cast_from(4.0_f64) * t23933 * t6517 - F::cast_from(4.0_f64) * t31734 * t4034 - F::cast_from(2.0_f64) * t8529 * t9348 - t112528 - t112535 - t112537 - t112542 - t114573 - t115195 - t115208 - t115210 - t115212 - t115217;
    t115222
}
