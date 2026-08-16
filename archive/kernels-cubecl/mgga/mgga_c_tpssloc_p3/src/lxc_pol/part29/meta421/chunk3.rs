//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1702/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1702<F: Float>(t16428: F, t3793: F, t1380: F, t16206: F, t12267: F, t1336: F, t1383: F, t16133: F, t16136: F, t16414: F, t16416: F, t16419: F, t16423: F, t1814: F, t1838: F, t1840: F, t3773: F, t3777: F, t3898: F, t3905: F, t3907: F, t3909: F, t5230: F, t5234: F, t5339: F, t5341: F, t5344: F, t544: F) -> F {
    let t16429 = t16428 * t3793;
    let t16433 = t1380 * t16206;
    let t16435 = -t12267 * t1838 - F::cast_from(2.0_f64) * t1336 * t16133 - t1336 * t16136 - F::cast_from(2.0_f64) * t1336 * t16416 - t1336 * t16423 + F::cast_from(2.0_f64) * t1336 * t16429 - t1336 * t16433 + F::cast_from(2.0_f64) * t1383 * t5230 + t16414 * t544 - F::cast_from(2.0_f64) * t16419 * t5344 + t1814 * t3909 + t1840 * t3773 - F::cast_from(2.0_f64) * t3777 * t5339 - F::cast_from(2.0_f64) * t3777 * t5341 + F::cast_from(2.0_f64) * t3898 * t5234 - t3905 * t5234 - t3907 * t5234;
    t16435
}
