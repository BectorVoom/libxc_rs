//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 802/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk802<F: Float>(t229: F, t4959: F, t1891: F, t5448: F, t653: F, t219: F, t518: F, t201: F, t673: F, t681: F, t1932: F, t1966: F, t207: F) -> (F, F, F, F, F, F, F) {
    let t5475 = t4959 * t229;
    let t5479 = 0.57895126195293126241e3 * t1891 * t653 * t5448;
    let t5486 = t518 * t219;
    let t5490 = t518 * t201;
    let t5503 = t673 * t681;
    let t5504 = t5503 * t1932;
    let t5507 = t207 * t1966;
    (t5475, t5479, t5486, t5490, t5503, t5504, t5507)
}
