//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 671/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk671<F: Float>(t1540: F, t325: F, t107: F, t1539: F, t209: F, t6247: F, t837: F, t874: F, t235: F, t6477: F, t117: F, t1915: F, t875: F, t899: F, t1614: F, t570: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28295 = t1540 * t325;
    let t28317 = t1539 * t107;
    let t29439 = t6247 * t209;
    let t29837 = t837 * t874;
    let t29838 = t235 * t29837;
    let t30080 = t6477 * t325;
    let t30174 = t28317 * t117;
    let t30177 = t1915 * t107;
    let t30204 = t899 * t875;
    let t30221 = t1540 * t117;
    let t30283 = t1614 * t570;
    (t28295, t29439, t29837, t29838, t30080, t30174, t30177, t30204, t30221, t30283)
}
