//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 998/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk998<F: Float>(t6693: F, t7748: F, t5047: F, t6613: F, t5077: F, t6496: F, t7754: F, t3338: F, t6555: F, t18463: F, t389: F, t1813: F, t5026: F, t1817: F, t4999: F, t3227: F, t6717: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t29054 = t7748 * t6693;
    let t29056 = t5047 * t6613;
    let t29057 = t7748 * t29056;
    let t29059 = t5077 * t6496;
    let t29060 = t7754 * t29059;
    let t29062 = t3338 * t6555;
    let t29063 = t7754 * t29062;
    let t29065 = t18463 * t389;
    let t29067 = t5026 * t1813;
    let t29069 = t4999 * t1817;
    let t29071 = t3227 * t6717;
    (t29054, t29056, t29057, t29059, t29060, t29062, t29063, t29065, t29067, t29069, t29071)
}
