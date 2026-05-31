//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1111/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1111<F: Float>(t2262: F, t6967: F, t3263: F, t3275: F, t7040: F, t792: F, t3276: F, t11020: F, t11540: F, t10622: F, t11629: F, t3579: F, t38283: F) -> (F, F, F, F, F) {
    let t39335 = t6967 * t2262;
    let t39338 = t3275 * t3263 * t39335 / F::cast_from(2.0_f64);
    let t39339 = t7040 * t792;
    let t39342 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t3276 * t39339;
    let t39344 = t11020 * t11540 / F::cast_from(4.0_f64);
    let t39347 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3275 * t11629 * t10622;
    let t39351 = t3579 * t38283 / F::cast_from(4.0_f64);
    (t39338, t39342, t39344, t39347, t39351)
}
