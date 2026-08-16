//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2034/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2034<F: Float>(t12725: F, t12823: F, t12841: F, t1774: F, t19456: F, t2040: F, t22574: F, t2312: F, t2314: F, t2364: F, t23918: F, t23929: F, t23938: F, t24008: F, t26114: F, t26558: F, t27150: F, t27188: F, t27219: F, t27226: F, t4028: F, t4034: F, t4037: F, t55962: F, t57802: F, t672: F, t7042: F, t7050: F, t7057: F, t7458: F, t7796: F, t7802: F, t7890: F, t92090: F, t9348: F) -> F {
    let t94061 = -F::cast_from(4.0_f64) * t23938 * t4037 - F::cast_from(2.0_f64) * t7042 * t12841 - F::cast_from(4.0_f64) * t2314 * t27150 - F::cast_from(2.0_f64) * t27188 * t2364 - F::cast_from(2.0_f64) * t55962 * t2040 - F::cast_from(4.0_f64) * t19456 * t7050 - F::cast_from(4.0_f64) * t4034 * t27219 - F::cast_from(4.0_f64) * t4028 * t23929 - F::cast_from(2.0_f64) * t12823 * t7802 - F::cast_from(4.0_f64) * t4034 * t27226 + F::cast_from(6.0_f64) * t22574 * t26558 * t57802 - t24008 * t1774 - t2312 * t7890 - F::cast_from(4.0_f64) * t12725 * t7057 - F::cast_from(4.0_f64) * t92090 * t672 - F::cast_from(2.0_f64) * t7458 * t23918 - F::cast_from(2.0_f64) * t9348 * t7796 - F::cast_from(4.0_f64) * t7458 * t23929 - F::cast_from(4.0_f64) * t26114 * t7057;
    t94061
}
