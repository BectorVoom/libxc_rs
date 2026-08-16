//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2675/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2675<F: Float>(t11922: F, t20104: F, t3115: F, t15618: F, t15984: F, t19477: F, t73: F, t1011: F, t15993: F, t18913: F, t18904: F, t53972: F) -> (F, F, F, F, F) {
    let t66362 = t3115 * t11922 * t20104;
    let t66376 = t15618 * t15984;
    let t66395 = t19477 * t73;
    let t66403 = t1011 * t15993 * t18913;
    let t66406 = t1011 * t53972 * t18904;
    (t66362, t66376, t66395, t66403, t66406)
}
