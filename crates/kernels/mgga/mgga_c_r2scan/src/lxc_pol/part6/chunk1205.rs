//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1205/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1205<F: Float>(t1669: F, t5469: F, t1416: F, t2036: F, t1376: F, t1693: F, t410: F, t5475: F, t424: F, t5717: F, t5439: F, t717: F, t1762: F, t224: F, t5960: F, t5222: F, t5376: F) -> (F, F, F, F, F, F, F, F) {
    let t22207 = t5469 * t1669;
    let t22210 = 120.0 * t1416 * t2036;
    let t22211 = t1376 * t1693;
    let t22217 = t410 * t5475;
    let t22219 = t424 * t5717;
    let t22222 = t424 * t717 * t5439;
    let t22227 = 0.11558335953042377059e2 * t1762 * t5960 * t224 * t5439;
    let t22228 = t5376 * t5222;
    (t22207, t22210, t22211, t22217, t22219, t22222, t22227, t22228)
}
