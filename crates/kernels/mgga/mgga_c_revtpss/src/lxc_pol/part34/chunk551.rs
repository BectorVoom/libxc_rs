//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 551/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk551<F: Float>(t512: F, t5569: F, t177: F, t1856: F, t762: F, t1468: F, t3874: F, t1711: F, t3881: F, t1892: F, t212: F, t1358: F) -> (F, F, F, F, F, F, F) {
    let t5570 = t512 * t5569;
    let t5571 = t1856 * t177;
    let t5572 = t5571 * t762;
    let t5574 = t3874 * t1468;
    let t5582 = t3881 * t1711;
    let t5599 = t212 * t1892;
    let t5600 = t5599 * t1358;
    (t5570, t5571, t5572, t5574, t5582, t5599, t5600)
}
