//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2056/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2056<F: Float>(t25273: F, t540: F, t1372: F, t2019: F, t9951: F, t2018: F, t9646: F, t9723: F, t26014: F, t2689: F, t807: F, t9714: F) -> (F, F, F, F, F, F) {
    let t94519 = t25273 * t540;
    let t94520 = t94519 * t1372;
    let t94522 = t2019 * t9951;
    let t94523 = F::cast_from(0.7558530601555998074e-1_f64) * t94522;
    let t94525 = t9646 * t2018 * t9723;
    let t94526 = F::cast_from(0.4016411544023718989e-6_f64) * t94525;
    let t94527 = t2689 * t26014;
    let t94530 = t807 * t2018 * t9714;
    (t94519, t94520, t94523, t94526, t94527, t94530)
}
