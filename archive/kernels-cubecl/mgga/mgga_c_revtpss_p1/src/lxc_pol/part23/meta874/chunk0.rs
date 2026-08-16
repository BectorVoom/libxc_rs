//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2776/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2776<F: Float>(t22126: F, t2689: F, t22130: F, t22081: F, t9962: F, t22276: F, t3989: F, t22281: F, t22056: F, t9765: F, t22021: F, t808: F, t9845: F) -> (F, F, F, F, F, F, F) {
    let t74491 = t2689 * t22126;
    let t74493 = t2689 * t22130;
    let t74498 = t9962 * t22081;
    let t74505 = t3989 * t22276;
    let t74507 = t3989 * t22281;
    let t74511 = t9765 * t22056;
    let t74522 = t9845 * t808 * t22021;
    (t74491, t74493, t74498, t74505, t74507, t74511, t74522)
}
