//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1452/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1452<F: Float>(t2266: F, t2854: F, t4933: F, t797: F, t8299: F, t481: F, t2858: F, t288: F, t7088: F, t5054: F, t97: F, t986: F, t19709: F, t19712: F, t20180: F, t23199: F, t25039: F, t25041: F, t25043: F, t25045: F, t27397: F) -> (F, F, F, F, F) {
    let t27400 = 3.0 * t2266 * t2854 * t4933;
    let t27401 = t8299 * t797;
    let t27404 = 9.0 * t2266 * t27401 * t481;
    let t27408 = 18.0 * t2858 * t288 * t7088 * t481;
    let t27412 = 6.0 * t97 * t5054 * t986 * t797;
    let t27413 = -t19709 - t27397 - t25039 + t25041 - t27400 + t19712 - t27404 - t27408 - t27412 + t23199 + t25043 + t20180 + t25045;
    (t27400, t27404, t27408, t27412, t27413)
}
