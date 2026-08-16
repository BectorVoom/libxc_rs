//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 583/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk583<F: Float>(t1149: F, t1150: F, t3384: F, t406: F, t409: F, t1134: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t1132: F) -> (F, F, F, F, F, F, F, F) {
    let t3385 = t1149 * t1149;
    let t3386 = t3385 * t1150;
    let t3388 = F::cast_from(2.0_f64) * t3384 * t3386;
    let t3390 = F::cast_from(1.0_f64) / t409 / t406;
    let t3391 = t1134 * t1134;
    let t3392 = t3390 * t3391;
    let t3394 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3356;
    let t3399 = t3394 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3358 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3365 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3370 + t3374 / F::cast_from(3.0_f64);
    let t3400 = t1132 * t3399;
    (t3385, t3386, t3388, t3390, t3391, t3392, t3399, t3400)
}
