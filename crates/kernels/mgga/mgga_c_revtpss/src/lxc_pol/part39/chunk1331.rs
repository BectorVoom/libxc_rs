//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1331/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1331<F: Float>(t2681: F, t64: F, t10207: F, t111: F, t116: F, t13424: F, t1501: F, t2371: F, t4245: F, t670: F, t1518: F, t2319: F, t4292: F, t648: F, t13514: F, t94: F) -> (F, F, F, F, F, F, F, F) {
    let t46089 = t64 * t2681;
    let t46157 = 1.0 / t10207 / t111;
    let t49686 = t13424 * t116;
    let t75485 = t1501 * t2371;
    let t75667 = t4245 * t670;
    let t98484 = t2319 * t1518;
    let t98487 = t648 * t4292;
    let t98535 = t94 * t13514;
    (t46089, t46157, t49686, t75485, t75667, t98484, t98487, t98535)
}
