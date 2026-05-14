//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 667/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk667<F: Float>(t107: F, t1539: F, t837: F, t874: F, t235: F, t1652: F, t321: F, t234: F, t833: F, t503: F, t325: F, t6477: F, t622: F, t794: F, t117: F, t875: F, t899: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28317 = t1539 * t107;
    let t29837 = t837 * t874;
    let t29838 = t235 * t29837;
    let t29892 = t1652 * t321;
    let t29927 = t234 * t833;
    let t29933 = t503 * t321;
    let t30080 = t6477 * t325;
    let t30137 = t622 * t794;
    let t30174 = t28317 * t117;
    let t30204 = t899 * t875;
    (t29837, t29838, t29892, t29927, t29933, t30080, t30137, t30174, t30204)
}
