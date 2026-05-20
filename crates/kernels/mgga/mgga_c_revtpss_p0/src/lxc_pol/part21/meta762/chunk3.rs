//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2705/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2705<F: Float>(t2289: F, t4288: F, t13455: F, t625: F, t10209: F, t1513: F, t2366: F, t28036: F, t31035: F, t46146: F, t46148: F, t46150: F, t46152: F, t46154: F, t46157: F, t49724: F, t49760: F, t49809: F, t655: F, t69: F) -> F {
    let t49817 = t2289 * t4288;
    let t49818 = F::new(11.0) / F::new(3.0) * t49817;
    let t49819 = t625 * t13455;
    let t49828 = -F::new(11.0) / F::new(3.0) * t46148 + t46154 / F::new(3.0) + t49724 - t69 * t655 * (t49760 + t49809) / F::new(8.0) + F::new(22.0) / F::new(3.0) * t46146 + F::new(2.0) * t46150 - F::new(2.0) * t46152 - t49818 + F::new(6.0) * t49819 + F::new(3.0) * t69 * t46157 * t1513 * t10209 - F::new(9.0) / F::new(4.0) * t31035 * t28036 * t2366;
    t49828
}
