//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1359/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1359<F: Float>(t26133: F, t571: F, t13226: F, t13250: F, t1456: F, t1458: F, t1464: F, t2038: F, t2045: F, t26094: F, t3: F, t4154: F, t4168: F, t575: F, t7319: F, t7337: F, t92556: F, t92559: F, t92563: F, t95119: F, t95125: F, t95127: F, t95176: F) -> F {
    let t95180 = t571 * t26133;
    let tv4rho3sigma0 = t3 * t575 * t95119 + t13226 * t2045 + t13250 * t2038 + F::cast_from(3.0_f64) * t1456 * t26133 + t1458 * t95176 + F::cast_from(3.0_f64) * t1464 * t26094 + F::cast_from(3.0_f64) * t4154 * t7337 + F::cast_from(3.0_f64) * t4168 * t7319 + F::cast_from(3.0_f64) * t92556 + F::cast_from(6.0_f64) * t92559 + F::cast_from(3.0_f64) * t92563 + F::cast_from(6.0_f64) * t95125 + F::cast_from(3.0_f64) * t95127 + F::cast_from(3.0_f64) * t95180;
    tv4rho3sigma0
}
