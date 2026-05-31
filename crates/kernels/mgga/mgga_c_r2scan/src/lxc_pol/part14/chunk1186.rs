//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1186/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1186<F: Float>(t12207: F, t12211: F, t12213: F, t12216: F, t3275: F, t3472: F, t40691: F, t11325: F, t11531: F, t11199: F, t11625: F, t3465: F, t40652: F) -> (F, F, F, F, F, F, F, F) {
    let t41147 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12207;
    let t41148 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12211;
    let t41149 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12213;
    let t41150 = F::cast_from(3.0_f64) * t12216;
    let t41153 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3275 * t3472 * t40691;
    let t41156 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t11325 * t11531;
    let t41158 = t3275 * t11199 * t11625;
    let t41160 = t3275 * t3465 * t40652;
    (t41147, t41148, t41149, t41150, t41153, t41156, t41158, t41160)
}
