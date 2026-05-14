//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1002/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1002<F: Float>(t119823: F, t126129: F, t119875: F, t33682: F, t31837: F, t33695: F, t31841: F, t33687: F, t686: F, t72: F, t120140: F, t31838: F, t33715: F, t845: F, t119859: F, t27279: F) -> (F, F, F, F, F, F, F) {
    let t126208 = t119823 * t126129;
    let t126210 = t119875 * t33682;
    let t126213 = t33695 * t31837;
    let t126214 = t126213 * t31841;
    let t126221 = t33687 * t72 * t686;
    let t126222 = t120140 * t126221;
    let t126226 = t31838 * t845 * t33715;
    let t126228 = t119859 * t27279;
    (t126208, t126210, t126214, t126221, t126222, t126226, t126228)
}
