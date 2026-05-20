//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3927/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3927<F: Float>(t1501: F, t2371: F, t4292: F, t21830: F, t625: F, t13509: F, t21820: F, t21876: F, t2339: F, t2340: F, t2366: F, t4263: F, t46143: F, t46157: F, t49698: F, t49700: F, t49702: F, t49704: F, t49724: F, t49817: F, t49819: F, t5891: F, t665: F, t69: F) -> (F, F, F) {
    let t75485 = t1501 * t2371;
    let t75494 = t4292 * t4292;
    let t75526 = t625 * t21830;
    let t75532 = F::new(88.0) / F::new(9.0) * t49700 - F::new(8.0) / F::new(3.0) * t49702 - F::new(4.0) / F::new(3.0) * t49704 + F::new(4.0) * t49819 + t46143 + F::new(2.0) / F::new(3.0) * t49724 - F::new(44.0) / F::new(9.0) * t49817 - F::new(3.0) / F::new(4.0) * t69 * t21820 * t2366 + t69 * t4263 * t13509 / F::new(2.0) + F::new(308.0) / F::new(27.0) * t49698 + F::new(3.0) * t69 * t46157 * t5891 * t2340 - F::new(4.0) / F::new(3.0) * t75526 + t69 * t2339 * t21876 * t665 / F::new(2.0);
    (t75485, t75494, t75532)
}
