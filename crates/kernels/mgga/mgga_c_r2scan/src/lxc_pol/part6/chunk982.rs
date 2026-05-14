//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 982/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk982<F: Float>(t2591: F, t7433: F, t360: F, t2572: F, t6127: F, t2573: F, t2133: F, t2598: F, t5123: F, t5126: F, t5130: F, t5144: F, t5150: F, t5164: F, t5170: F, t5175: F, t5179: F, t5183: F, t5186: F, t5189: F, t7419: F, t7430: F) -> (F, F, F, F, F, F, F) {
    let t7434 = t7433 * t2591;
    let t7435 = t360 * t7434;
    let t7438 = t2572 * t6127;
    let t7439 = t360 * t7438;
    let t7442 = t7433 * t2573;
    let t7443 = t360 * t7442;
    let t7446 = 0.11708928647259339622e0 * t5123 - 0.97574405393827830186e-2 * t5126 - 0.58218257753910989057e-2 * t5130 - 0.23115257973478049502e0 * t5144 + 0.16262400898971305031e-3 * t5150 - 0.28914548798370980346e-3 * t7419 - 0.17465477326173296717e-1 * t5164 + 0.27439371595564631661e-2 * t5170 - 0.54878743191129263322e-2 * t5175 + 0.1358426014257923078e0 * t5179 + 0.4075278042773769234e0 * t5183 - 0.11643651550782197811e-1 * t5186 - 0.34930954652346593434e-1 * t5189 + 0.43341108700271342816e-1 * t2133 * t7430 + 0.17336443480108537126e0 * t2598 * t7435 + 0.86682217400542685632e-1 * t2598 * t7439 + 0.86682217400542685632e-1 * t2133 * t7443;
    (t7434, t7435, t7438, t7439, t7442, t7443, t7446)
}
