//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 744/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk744<F: Float>(t1762: F, t5200: F, t1767: F, t1978: F, t1818: F, t377: F, t1983: F, t1763: F, t1949: F, t1734: F, t1771: F, t124: F, t704: F, t706: F, t1672: F, t584: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5202 = 0.97592231702715658578e-1 * t1762 * t5200;
    let t5203 = t1767 * t1978;
    let t5205 = 0.48159733137676571079e0 * t1762 * t5203;
    let t5206 = t377 * t1818;
    let t5207 = t5206 * t1983;
    let t5209 = 0.28518989949414381017e2 * t1762 * t5207;
    let t5210 = t1763 * t1949;
    let t5212 = 0.65061487801810439052e-1 * t1762 * t5210;
    let t5213 = t1771 * t1734;
    let t5215 = t124 * t704;
    let t5216 = t5215 * t706;
    let t5218 = 0.43374325201206959369e-1 * t1762 * t5216;
    let t5219 = t584 * t1672;
    (t5202, t5203, t5205, t5206, t5207, t5209, t5210, t5212, t5213, t5215, t5216, t5218, t5219)
}
