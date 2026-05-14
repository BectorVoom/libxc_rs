//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 637/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk637<F: Float>(t4751: F, t4733: F, t4736: F, t4739: F, t4831: F, t4832: F, t4833: F, t4834: F, t401: F, t384: F, t4824: F, t1483: F, t1466: F, t1477: F, t402: F, t4741: F) -> (F, F, F, F, F) {
    let t4835 = 0.36514074074074074075e0 * t4751;
    let t4836 = -0.25319e1 * t4733 + 0.16879333333333333333e1 * t4736 - 0.19692555555555555555e1 * t4739 - t4831 + t4832 - t4833 - t4834 - t4835;
    let t4837 = t4836 * t401;
    let t4838 = t384 * t4837;
    let t4839 = 1.0 * t4838;
    let t4840 = t4824 * t401;
    let t4841 = t1483 * t4840;
    let t4842 = 6.0 * t4841;
    let t4844 = t1466 * t402 * t1477;
    let t4845 = 6.0 * t4844;
    let t4849 = 0.93932222222222222223e0 * t4741;
    (t4835, t4839, t4842, t4845, t4849)
}
