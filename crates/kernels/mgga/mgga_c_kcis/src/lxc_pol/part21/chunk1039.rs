//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1039/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1039<F: Float>(t2169: F, t26393: F, t2210: F, t2794: F, t2146: F, t2537: F, t2539: F, t7612: F, t8522: F, t235: F, t3703: F, t2801: F, t441: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26394 = t2169 * t26393;
    let t26395 = t26394 / F::cast_from(8.0_f64);
    let t26396 = t2794 * t2210;
    let t26397 = t26396 / F::cast_from(8.0_f64);
    let t26398 = t2146 * t2537;
    let t26399 = t26398 * t2539;
    let t26400 = F::cast_from(2.0_f64) * t26399;
    let t26401 = t8522 * t7612;
    let t26402 = F::cast_from(4.0_f64) * t26401;
    let t26403 = t235 * t3703;
    let t26404 = t2169 * t26403;
    let t26405 = t26404 / F::cast_from(16.0_f64);
    let t26406 = t2801 * t441;
    (t26395, t26397, t26398, t26399, t26400, t26401, t26402, t26405, t26406)
}
