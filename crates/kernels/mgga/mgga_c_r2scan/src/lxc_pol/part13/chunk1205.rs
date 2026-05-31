//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1205/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1205<F: Float>(t3446: F, t3447: F, t40453: F, t874: F, t122: F, t3434: F, t3437: F, t3579: F, t38289: F, t1563: F, t2867: F, t10997: F, t3275: F) -> (F, F, F, F) {
    let t40456 = t3446 * t3447 * t40453 * t874;
    let t40457 = F::cast_from(0.30487649791575028314e-3_f64) * t40456;
    let t40460 = t3434 * t3437 * t40453 * t122;
    let t40461 = F::cast_from(0.43368970657079495312e-4_f64) * t40460;
    let t40463 = t3579 * t38289 / F::cast_from(4.0_f64);
    let t40464 = t2867 * t1563;
    let t40467 = F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t3275 * t10997 * t40464;
    (t40457, t40461, t40463, t40467)
}
