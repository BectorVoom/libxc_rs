//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1283/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1283<F: Float>(t29068: F, t46862: F, t1495: F, t2681: F, t12001: F, t29162: F, t29194: F, t8392: F, t29199: F, t29204: F, t29209: F, t6353: F, t848: F, t1882: F, t29174: F, t112391: F, t112410: F, t1255: F, t15225: F, t15255: F, t15371: F, t1901: F, t25135: F, t2749: F, t28719: F, t29293: F, t296: F, t446: F, t840: F, t871: F, t875: F, t99238: F) -> (F,) {
    let t114818 = t46862 * t29068;
    let t114820 = t2681 * t1495;
    let t114827 = t12001 * t29162;
    let t114837 = 2.0 / 27.0 * t8392 * t29194;
    let t114839 = 4.0 / 27.0 * t8392 * t29199;
    let t114841 = 4.0 / 27.0 * t8392 * t29204;
    let t114843 = 4.0 / 81.0 * t8392 * t29209;
    let t114847 = t848 * t6353;
    let t114852 = 2.0 / 9.0 * t1882 * t29174;
    let t114853 = 2.0 / 3.0 * t446 * t840 * t2749 * t29293 + 2.0 / 3.0 * t446 * t840 * t871 * t28719 * t875 + 22.0 / 27.0 * t114818 - 4.0 / 3.0 * t1901 * t114820 * t15371 + 4.0 / 3.0 * t446 * t296 * t112410 - 22.0 / 27.0 * t114827 + 2.0 / 3.0 * t446 * t296 * t112391 - t446 * t840 * t1255 * t25135 / 3.0 + t114837 + t114839 + t114841 - t114843 - 2.0 / 9.0 * t1901 * t99238 * t15225 - 4.0 / 9.0 * t1901 * t114847 * t15255 + t114852;
    (t114853,)
}
