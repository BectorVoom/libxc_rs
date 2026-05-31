//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1213/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1213<F: Float>(t3275: F, t3465: F, t42384: F, t42403: F, t11345: F, t12422: F, t11523: F, t12203: F, t11625: F, t12056: F, t3472: F, t42428: F) -> (F, F, F, F, F, F) {
    let t44108 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3275 * t3465 * t42384;
    let t44110 = t3275 * t3465 * t42403;
    let t44113 = t12422 * t11345 / F::cast_from(4.0_f64);
    let t44115 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t11523 * t12203;
    let t44117 = t3275 * t12056 * t11625;
    let t44120 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3275 * t3472 * t42428;
    (t44108, t44110, t44113, t44115, t44117, t44120)
}
