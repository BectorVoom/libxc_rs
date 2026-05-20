//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1591/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1591<F: Float>(t21820: F, t22628: F, t2339: F, t4263: F, t46143: F, t46157: F, t49698: F, t5915: F, t655: F, t69: F, t75540: F, t75639: F, t75822: F, t75831: F, t75843: F, t86981: F, t86988: F, t87046: F) -> F {
    let t87050 = t46143 + F::new(616.0) / F::new(27.0) * t49698 + F::new(44.0) / F::new(3.0) * t75639 - F::new(22.0) / F::new(3.0) * t75540 + F::new(8.0) * t75822 - F::new(8.0) * t75831 + F::new(4.0) / F::new(3.0) * t75843 + F::new(3.0) * t69 * t46157 * t86981 - F::new(9.0) / F::new(2.0) * t69 * t21820 * t5915 + F::new(3.0) / F::new(4.0) * t69 * t2339 * t86988 + t69 * t4263 * t22628 - t69 * t655 * t87046 / F::new(8.0);
    t87050
}
