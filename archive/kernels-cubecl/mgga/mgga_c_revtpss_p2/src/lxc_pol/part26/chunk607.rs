//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 607/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk607<F: Float>(t30: F, t33: F, t2630: F, t3869: F, t1337: F, t2619: F, t514: F, t1344: F, t2257: F, t3834: F, t517: F, t1348: F, t3351: F, t3842: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3871 = F::cast_from(0.10843581300301739842e-1_f64) * t3869 * t2630;
    let t3873 = F::cast_from(0.24415263074675393405e-3_f64) * t1337 * t2619;
    let t3874 = F::cast_from(1.0_f64) / t514;
    let t3880 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3874 * t3834 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1344 * t2257);
    let t3881 = F::cast_from(1.0_f64) / t517;
    let t3887 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3881 * t3842 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1348 * t3351);
    (t3871, t3873, t3874, t3880, t3881, t3887)
}
