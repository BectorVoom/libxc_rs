//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 888/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk888<F: Float>(t11017: F, t106: F, t1550: F, t97: F, t3271: F, t10918: F, t3262: F, t3264: F, t10983: F, t10988: F, t10991: F, t10996: F, t11001: F, t11006: F, t11008: F, t11014: F) -> (F, F, F, F, F) {
    let t11018 = 0.1951603679568577289e-3 * t11017;
    let t11020 = t97 * t106 * t1550;
    let t11021 = t11020 * t3271;
    let t11022 = t11021 / 4.0;
    let t11024 = t3262 * t10918 * t3264;
    let t11025 = 3.0 / 2.0 * t11024;
    let t11026 = -t10983 - t10988 + t10991 + t10996 - t11001 + t11006 - 0.81300399444200075504e-3 * t11008 + t11014 - t11018 - t11022 - t11025;
    (t11018, t11020, t11022, t11025, t11026)
}
