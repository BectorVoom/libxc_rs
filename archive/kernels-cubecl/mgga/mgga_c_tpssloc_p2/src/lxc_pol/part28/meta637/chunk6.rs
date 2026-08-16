//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2038/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2038<F: Float>(t2098: F, t2319: F, t111: F, t7945: F, t12524: F, t12813: F, t1458: F, t16535: F, t16538: F, t16541: F, t20173: F, t2039: F, t23917: F, t24465: F, t27170: F, t27273: F, t27276: F, t27281: F, t3938: F, t3941: F, t4072: F, t45560: F, t55341: F, t55571: F, t577: F, t66940: F, t7056: F, t7230: F, t7801: F, t7956: F, t94106: F) -> F {
    let t94165 = t2098 * t2319;
    let t94170 = t7945 * t111;
    let t94202 = F::cast_from(27.0_f64) * t45560 * t7956 + F::cast_from(0.135e2_f64) * t55341 * t2039 + F::cast_from(27.0_f64) * t94165 * t1458 + F::cast_from(0.135e2_f64) * t7230 * t12813 + F::cast_from(27.0_f64) * t94170 * t2319 + F::cast_from(54.0_f64) * t66940 * t7956 + F::cast_from(27.0_f64) * t3941 * t23917 * t1458 + F::cast_from(54.0_f64) * t3941 * t7056 * t4072 + F::cast_from(0.45e1_f64) * t94106 * t577 + F::cast_from(54.0_f64) * t24465 * t16538 + F::cast_from(27.0_f64) * t24465 * t16541 + F::cast_from(54.0_f64) * t12524 * t27281 + F::cast_from(27.0_f64) * t3941 * t2039 * t12813 + F::cast_from(27.0_f64) * t55571 * t7956 + F::cast_from(54.0_f64) * t20173 * t27273 + F::cast_from(54.0_f64) * t20173 * t27276 + F::cast_from(27.0_f64) * t16535 * t7801 + F::cast_from(27.0_f64) * t3938 * t27170;
    t94202
}
