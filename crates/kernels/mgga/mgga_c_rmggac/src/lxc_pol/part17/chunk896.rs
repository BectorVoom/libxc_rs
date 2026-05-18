//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 896/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk896<F: Float>(t1614: F, t1970: F, t1971: F, t209: F, t511: F, t605: F, t1494: F, t558: F, t10030: F, t7255: F, t1652: F, t515: F) -> (F, F, F, F) {
    let t45012 = t1970 * t1971 * t511 * t1614 * t605 * t209;
    let t45018 = t1970 * t1971 * t511 * t558 * t1494 * t209;
    let t45020 = t7255 * t10030;
    let t45026 = t1970 * t1971 * t515 * t1652 * t605 * t209;
    (t45012, t45018, t45020, t45026)
}
