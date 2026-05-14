//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 734/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk734<F: Float>(t1510: F, t406: F, t1512: F, t410: F, t1411: F, t732: F, t1524: F, t1384: F, t4811: F, t4816: F, t234: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5021 = t406 * t1510;
    let t5022 = 12.0 * t5021;
    let t5025 = t406 * t1512;
    let t5026 = 24.0 * t5025;
    let t5027 = t410 * t1512;
    let t5028 = 24.0 * t5027;
    let t5029 = t732 * t1411;
    let t5030 = 0.17544670867903938621e1 * t5029;
    let t5034 = t732 * t1524;
    let t5035 = 0.35089341735807877242e1 * t5034;
    let t5037 = t4816 * t4811 * t1384;
    let t5038 = t234 * t5037;
    let t5039 = 0.10389515463408878255e3 * t5038;
    (t5021, t5022, t5025, t5026, t5027, t5028, t5029, t5030, t5034, t5035, t5037, t5038, t5039)
}
