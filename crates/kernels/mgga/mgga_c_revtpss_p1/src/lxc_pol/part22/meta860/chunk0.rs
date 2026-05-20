//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3009/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3009<F: Float>(t14786: F, t231: F, t40834: F, t854: F, t14833: F, t236: F, t2453: F, t9794: F, t125: F, t14662: F, t10777: F, t14671: F, t14917: F, t40725: F) -> (F, F, F, F, F) {
    let t50451 = t14786 * t231;
    let t50453 = t40834 * t854 * t50451;
    let t50457 = t2453 * t236 * t9794 * t14833;
    let t50459 = t125 * t14662;
    let t50466 = t10777 * t40725 * t14671 * t14917;
    (t50451, t50453, t50457, t50459, t50466)
}
