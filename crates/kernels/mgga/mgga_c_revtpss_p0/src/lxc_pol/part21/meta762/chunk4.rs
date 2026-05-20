//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2706/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2706<F: Float>(t114: F, t10208: F, t10254: F, t13458: F, t13509: F, t2339: F, t2340: F, t2366: F, t4263: F, t4287: F, t46143: F, t46144: F, t49698: F, t49701: F, t49702: F, t49704: F, t49828: F, t665: F, t69: F) -> F {
    let t115 = F::new(1.0) < t114;
    let t49830 = piecewise3::<F>(t115, F::new(0.0), F::new(154.0) / F::new(27.0) * t49698 + t49701 - F::new(4.0) * t49702 - F::new(2.0) * t49704 - F::new(9.0) / F::new(4.0) * t69 * t10208 * t4287 * t2340 + F::new(3.0) / F::new(4.0) * t69 * t2339 * t13509 * t665 + F::new(3.0) / F::new(4.0) * t69 * t13458 * t2366 + t69 * t4263 * t10254 / F::new(4.0) + t46143 + F::new(154.0) / F::new(9.0) * t46144 + t49828);
    t49830
}
