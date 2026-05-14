//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 905/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk905<F: Float>(t10536: F, t10592: F, t10593: F, t10594: F, t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5025: F, t5178: F, t5186: F, t7047: F, t8850: F, t8852: F, t10534: F, t124: F) -> (F, F, F, F, F) {
    let t10595 = t10536 + t4996 + t5005 - t5011 - t10592 - t10593 + t10594 + t5019 - t5022 + t5178 + t5186 + t5025;
    let t10596 = 0.32530743900905219526e-1 * t7047;
    let t10597 = 12.0 * t8850;
    let t10598 = 12.0 * t8852;
    let t10600 = 0.19751673498613801407e-1 * t10534 * t124;
    (t10595, t10596, t10597, t10598, t10600)
}
