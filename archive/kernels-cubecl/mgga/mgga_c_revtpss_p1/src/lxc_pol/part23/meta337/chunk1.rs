//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1638/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1638<F: Float>(t136: F, t1883: F, t2457: F, t10139: F, t13926: F, t543: F) -> (F, F, F, F) {
    let t14219 = t1883 * t136;
    let t14220 = t14219 * t2457;
    let t14221 = t10139 * t14220;
    let t14224 = t13926 * t543;
    (t14219, t14220, t14221, t14224)
}
