//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1312/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1312<F: Float>(t27932: F, t47300: F, t26009: F, t9802: F, t26004: F, t3961: F, t7252: F, t9700: F, t94456: F, t94460: F, t94462: F, t94464: F, t94466: F, t94468: F, t94472: F, t94474: F, t94477: F, t94479: F) -> F {
    let t94481 = t27932 * t47300;
    let t94483 = t9802 * t26009;
    let t94484 = F::cast_from(0.91476005056713590805e-4_f64) * t94483;
    let t94485 = t26004 * t3961;
    let t94487 = t7252 * t9700;
    let t94489 = -F::cast_from(0.12004725073059526352e-1_f64) * t94456 - F::cast_from(0.34013387707001991332e-1_f64) * t94460 - F::cast_from(0.42874018118069736972e-3_f64) * t94462 + F::cast_from(0.25724410870841842184e-1_f64) * t94464 - F::cast_from(0.42874018118069736972e-3_f64) * t94466 - F::cast_from(0.76230004213927992339e-4_f64) * t94468 - t94472 + t94474 - t94477 + F::cast_from(0.60984003371142393869e-4_f64) * t94479 + F::new(3.0) / F::new(16.0) * t94481 + t94484 + F::new(7.0) / F::new(48.0) * t94485 - t94487 / F::new(48.0);
    t94489
}
