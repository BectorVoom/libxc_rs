//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1797/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1797<F: Float>(t13226: F, t13250: F, t1456: F, t1458: F, t1464: F, t3: F, t39397: F, t39399: F, t39401: F, t39403: F, t4154: F, t4168: F, t47693: F, t47728: F, t575: F) -> F {
    let tv4rho40 = t3 * t47693 * t575 + F::new(4.0) * t13226 * t1464 + F::new(4.0) * t13250 * t1456 + t1458 * t47728 + F::new(6.0) * t4154 * t4168 + F::new(4.0) * t39397 + F::new(12.0) * t39399 + F::new(12.0) * t39401 + F::new(4.0) * t39403;
    tv4rho40
}
