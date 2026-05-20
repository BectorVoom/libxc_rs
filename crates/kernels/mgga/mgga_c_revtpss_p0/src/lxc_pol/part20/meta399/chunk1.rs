//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1480/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1480<F: Float>(t3059: F, t3075: F, t41306: F, t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F) -> (F, F) {
    let t42001 = t3059 * t3075;
    let t42013 = F::cast_from(0.86419753086419753087e-1_f64) * t41306;
    let t42018 = F::cast_from(0.66666666666666666668e-1_f64) * t41308 + F::new(0.2e0) * t41312 - F::new(0.3e0) * t41316 + F::cast_from(0.50000000000000000001e-1_f64) * t41320 + F::cast_from(0.19999999999999999999e0_f64) * t41323 - F::cast_from(0.16666666666666666666e-1_f64) * t41327 + t42013 - F::cast_from(0.22222222222222222222e-1_f64) * t41330 - F::cast_from(0.14814814814814814815e-1_f64) * t41332 + F::cast_from(0.11111111111111111111e-1_f64) * t41334 + F::cast_from(0.12345679012345679012e-1_f64) * t41336;
    (t42001, t42018)
}
