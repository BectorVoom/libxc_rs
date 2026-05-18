//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 740/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk740<F: Float>(t2545: F, t2558: F, t2551: F, t8546: F, t22: F, t728: F, t736: F, t126: F, t2379: F, t15: F, t684: F, t762: F) -> (F, F, F, F, F) {
    let t9113 = t2545 * t2558;
    let t9118 = t2551 * t8546;
    let t9120 = t22 * t736 * t728;
    let t9123 = t2379 * t126;
    let t9124 = t9123 * t15;
    let t9129 = t684 * t762;
    (t9113, t9118, t9120, t9124, t9129)
}
