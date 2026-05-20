//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1164/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1164<F: Float>(t68: F, t8442: F, t119457: F, t32798: F, t33280: F, t644: F, t8621: F, t624: F, t84: F, t640: F, t8737: F, t36: F) -> (F, F, F, F, F, F) {
    let t124193 = t8442 * t68;
    let t124200 = t119457 * t68;
    let t124210 = t32798 * t8621 * t33280 * t644;
    let t124217 = t84 * t624;
    let t124220 = t8737 * t8621 * t124217 * t640;
    let t124235 = t624 * t36;
    (t124193, t124200, t124210, t124217, t124220, t124235)
}
