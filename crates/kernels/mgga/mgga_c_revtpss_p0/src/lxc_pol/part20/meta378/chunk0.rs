//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1370/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1370<F: Float>(t10871: F, t40262: F, t14917: F, t2475: F, t2661: F, t2662: F, t836: F, t2749: F, t40378: F, t2430: F, t853: F, t837: F) -> (F, F, F, F, F) {
    let t40537 = t40262 * t10871;
    let t40549 = t2661 * t2662 * t2475 * t836 * t14917;
    let t40553 = t2661 * t2662 * t40378 * t2749;
    let t40555 = t853 * t2430;
    let t40558 = t2661 * t2662 * t40555 * t837;
    (t40537, t40549, t40553, t40555, t40558)
}
