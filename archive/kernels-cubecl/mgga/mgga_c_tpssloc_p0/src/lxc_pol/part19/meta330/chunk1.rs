//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1178/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1178<F: Float>(t3736: F, t40018: F, t12012: F, t12220: F, t16101: F, t210: F, t213: F, t214: F, t221: F, t3719: F, t3733: F, t3734: F, t39622: F, t40343: F, t40347: F, t40350: F, t40351: F, t40356: F, t40360: F, t40366: F, t40372: F, t40376: F, t5195: F) -> F {
    let t40387 = t40018 * t3736;
    let t40389 = -t40343 + t40347 + t40350 - F::cast_from(0.79999999999999999997e-1_f64) * t40351 - F::cast_from(0.29999999999999999998e-1_f64) * t40356 + F::cast_from(0.99999999999999999996e-2_f64) * t40360 + F::cast_from(0.19999999999999999999e-1_f64) * t5195 * t221 * t12220 * t12012 - F::cast_from(0.13999999999999999999e0_f64) * t40366 + F::cast_from(0.11111111111111111111e-2_f64) * t40372 - F::cast_from(0.29999999999999999998e-1_f64) * t40376 - F::cast_from(0.11999999999999999999e0_f64) * t16101 * t221 * t213 * t3734 * t3719 + F::cast_from(0.14999999999999999999e-1_f64) * t3733 * t210 * t214 * t39622 + F::cast_from(0.23333333333333333332e0_f64) * t40387;
    t40389
}
