//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2803/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2803<F: Float>(t13458: F, t21820: F, t21876: F, t22589: F, t22628: F, t2339: F, t31035: F, t4263: F, t4287: F, t46157: F, t5915: F, t655: F, t665: F, t69: F, t75542: F, t75822: F, t75831: F, t75833: F, t75843: F, t75887: F, t75924: F) -> F {
    let t75929 = t75542 + F::new(2.0) * t75822 + F::new(3.0) * t69 * t46157 * t22589 * t665 - F::new(9.0) / F::new(4.0) * t69 * t21820 * t4287 - F::new(2.0) * t75831 - F::new(9.0) / F::new(4.0) * t31035 * t75833 * t665 + F::new(3.0) / F::new(4.0) * t69 * t13458 * t5915 + F::new(3.0) / F::new(4.0) * t69 * t4263 * t21876 + t75843 / F::new(3.0) + t69 * t2339 * t22628 * t665 / F::new(4.0) - t69 * t655 * (t75887 + t75924) / F::new(8.0);
    t75929
}
