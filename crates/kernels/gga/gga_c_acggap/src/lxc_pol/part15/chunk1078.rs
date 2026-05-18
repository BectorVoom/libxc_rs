//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1078/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1078<F: Float>(t32130: F, t36495: F, t38086: F, t7963: F, t7965: F, t23688: F, t7942: F, t8306: F, t33509: F, t33551: F, t33787: F, t2131: F, t2147: F, t2394: F, t847: F) -> (F, F, F, F, F, F, F) {
    let t38415 = F::new(0.34694512752820797848e1) * t32130 * t38086 * t36495;
    let t38418 = F::new(0.17347256376410398924e1) * t7963 * t38086 * t7965;
    let t38430 = F::new(0.17347256376410398924e1) * t7942 * t8306 * t23688;
    let t38432 = t7942 * t8306 * t33509;
    let t38441 = F::new(0.17347256376410398924e1) * t7963 * t8306 * t33551;
    let t38443 = t7963 * t8306 * t33787;
    let t38453 = t2131 * t2147 * t2394 * t847;
    (t38415, t38418, t38430, t38432, t38441, t38443, t38453)
}
