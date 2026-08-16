//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2024/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2024<F: Float>(t7365: F, t85660: F, t131: F, t467: F, t50: F, t82510: F, t10469: F, t461: F, t11721: F, t3032: F, t3508: F, t7368: F) -> (F, F, F, F, F, F) {
    let t85952 = t85660 * t7365;
    let t85963 = t50 * t82510 * t131 * t467;
    let t85964 = t461 * t10469;
    let t85966 = t3032 * t11721;
    let t85972 = t3032 * t3508;
    let t85986 = t85660 * t7368;
    (t85952, t85963, t85964, t85966, t85972, t85986)
}
