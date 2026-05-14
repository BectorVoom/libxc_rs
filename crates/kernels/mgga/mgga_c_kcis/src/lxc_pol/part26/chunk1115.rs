//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1115/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1115<F: Float>(t4527: F, t7671: F, t1655: F, t26654: F, t28311: F, t28314: F, t28317: F, t28320: F, t28323: F, t17396: F, t491: F, t17449: F, t11825: F, t27543: F, t1928: F, t4248: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t97561 = 2.0 * t4527 * t7671;
    let t97601 = t1655 * t26654;
    let t97622 = t28311 / 8.0;
    let t97623 = t28314 / 8.0;
    let t97624 = t28317 / 8.0;
    let t97625 = t28320 / 8.0;
    let t97626 = t28323 / 8.0;
    let t97681 = t17396 * t491;
    let t97701 = t17449 * t491;
    let t97706 = t11825 * t27543;
    let t97727 = t4248 * t1928;
    (t97561, t97601, t97622, t97623, t97624, t97625, t97626, t97681, t97701, t97706, t97727)
}
