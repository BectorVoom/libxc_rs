//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 382/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk382<F: Float>(t1751: F, t466: F, t1734: F, t491: F, t1246: F, t493: F, t1244: F, t1729: F, t470: F, t494: F, t1241: F, t1238: F, t1721: F, t498: F) -> (F, F, F, F, F, F, F) {
    let t1752 = t466 * t1751;
    let t1755 = t491 * t1734;
    let t1756 = t1755 * t1246;
    let t1758 = t493 * t1751;
    let t1760 = t1244 * t1756 + t1729 * t494 + t1758 * t470;
    let t1761 = t1241 * t1760;
    let t1763 = -t1238 * t1761 + t1721 * t498 + t1752 * t498;
    (t1752, t1755, t1756, t1758, t1760, t1761, t1763)
}
