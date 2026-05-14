//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 771/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk771<F: Float>(t2157: F, t5148: F, t5147: F, t4933: F, t552: F, t551: F, t506: F, t529: F, t1234: F, t788: F, t2207: F, t785: F, t2169: F, t2173: F, t5098: F, t5101: F, t5106: F, t5108: F, t5111: F, t5117: F, t5121: F, t5123: F, t5126: F, t5130: F, t5136: F, t5139: F, t5144: F, t527: F, t566: F) -> (F, F, F, F, F, F, F, F) {
    let t5149 = t5148 * t2157;
    let t5150 = t5147 * t5149;
    let t5154 = t552 * t4933;
    let t5155 = t551 * t5154;
    let t5158 = t506 * t4933;
    let t5159 = t529 * t5158;
    let t5162 = t788 * t1234;
    let t5164 = t2207 * t785 * t5162;
    let t5166 = 0.20958572791407956061e0 * t5098 - 0.4939086887201633699e-1 * t5101 + 0.34930954652346593433e-1 * t5106 - 0.7801399566048841707e0 * t5108 * t5111 + 0.82318114786693894983e-2 * t5117 + 0.87816964854445047168e-1 * t5121 + 0.17563392970889009434e0 * t5123 - 0.29272321618148349056e-1 * t5126 - 0.17465477326173296717e-1 * t5130 - 0.2600466522016280569e0 * t5136 * t5139 - 0.69345773920434148506e0 * t5144 + 0.24393601348456957547e-3 * t5150 - 0.39006997830244208535e0 * t2169 * t2173 - 0.13002332610081402845e0 * t566 * t5155 - 0.54878743191129263322e-1 * t527 * t5159 - 0.52396431978519890151e-1 * t5164;
    (t5149, t5150, t5155, t5158, t5159, t5162, t5164, t5166)
}
