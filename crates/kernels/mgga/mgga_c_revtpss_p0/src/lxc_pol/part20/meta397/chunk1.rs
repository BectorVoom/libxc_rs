//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1466/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1466<F: Float>(t41308: F, t41312: F, t41320: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t41433: F, t41436: F, t41439: F, t41441: F) -> F {
    let t41732 = -F::new(0.41318e1) * t41365 + F::cast_from(0.13772666666666666667e1_f64) * t41367 + F::new(0.41318e1) * t41308 + F::new(0.123954e2) * t41312 + F::new(0.309885e1) * t41320 - F::new(0.103295e1) * t41327 - F::cast_from(0.13772666666666666666e1_f64) * t41330 - F::cast_from(0.91817777777777777776e0_f64) * t41332 + F::cast_from(0.68863333333333333332e0_f64) * t41334 + F::cast_from(0.76514814814814814814e0_f64) * t41336 - F::new(0.104195e0) * t41433 + F::new(0.250068e1) * t41436 + F::new(0.62517e0) * t41439 + F::cast_from(0.12349037037037037037e1_f64) * t41441;
    t41732
}
