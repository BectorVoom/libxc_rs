//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1872/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1872<F: Float>(t5697: F, t94429: F, t5701: F, t27928: F, t9775: F, t13775: F, t25986: F, t2661: F, t25978: F, t5614: F, t5622: F, t94443: F) -> (F, F, F, F, F, F) {
    let t98128 = t94429 * t5697;
    let t98130 = t94429 * t5701;
    let t98141 = t9775 * t27928;
    let t98144 = t2661 * t25986 * t13775;
    let t98146 = t25978 * t5614;
    let t98148 = t94443 * t5622;
    (t98128, t98130, t98141, t98144, t98146, t98148)
}
