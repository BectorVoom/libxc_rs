//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2689/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2689<F: Float>(t20020: F, t3224: F, t1025: F, t127: F, t19768: F, t371: F, t225: F, t64686: F, t366: F, t64907: F, t19773: F, t3215: F) -> (F, F, F, F, F) {
    let t67493 = t3224 * t20020;
    let t67499 = t1025 * t371 * t127 * t19768;
    let t67501 = t64686 * t225;
    let t67516 = t64907 * t366;
    let t67521 = t19773 * t3215;
    (t67493, t67499, t67501, t67516, t67521)
}
