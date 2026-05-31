//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1239/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1239<F: Float>(t1013: F, t1120: F, t11217: F, t11223: F, t12256: F, t12259: F, t1292: F, t1295: F, t1300: F, t19203: F, t2394: F, t2400: F, t3506: F, t3735: F, t38783: F, t38839: F, t6693: F, t829: F, t8398: F, t8409: F, t8412: F, t8415: F) -> F {
    let t41854 = -F::cast_from(0.768e1_f64) * t6693 * t12256 * t829 - F::cast_from(0.768e1_f64) * t6693 * t12259 * t829 - F::cast_from(0.384e1_f64) * t6693 * t3735 * t1292 - F::cast_from(0.1536e2_f64) * t19203 * t3735 * t1295 - F::cast_from(0.768e1_f64) * t38839 * t2400 - F::cast_from(0.768e1_f64) * t11223 * t8412 - F::cast_from(0.384e1_f64) * t11223 * t8415 - F::cast_from(0.1536e2_f64) * t38783 * t8409 - F::cast_from(0.128e1_f64) * t1300 * t11217 * t1013 - F::cast_from(0.256e1_f64) * t1300 * t3506 * t2394 - F::cast_from(0.128e1_f64) * t1300 * t1120 * t8398;
    t41854
}
