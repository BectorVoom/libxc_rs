//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1057/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1057<F: Float>(t11921: F, t247: F, t31913: F, t31914: F, t3143: F, t36865: F, t8514: F, t31927: F, t7150: F, t3268: F, t8513: F, t93488: F) -> (F, F, F, F) {
    let t120647 = t31913 * t247 * t11921 * t31914;
    let t120653 = t36865 * t3143;
    let t120654 = t8514 * t120653;
    let t120664 = t7150 * t31927;
    let t120671 = t8513 * t93488 * t3268;
    (t120647, t120654, t120664, t120671)
}
