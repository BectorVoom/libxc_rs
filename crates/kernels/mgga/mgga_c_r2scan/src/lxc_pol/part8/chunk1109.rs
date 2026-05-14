//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1109/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1109<F: Float>(t20481: F, t495: F, t551: F, t574: F, t2195: F, t2597: F, t6148: F, t1603: F, t2116: F, t4888: F, t5: F, t511: F, t7: F, t512: F, t57: F, t6101: F) -> (F, F, F, F, F) {
    let t20484 = t574 * t551 * t20481 * t495;
    let t20499 = t2195 * t2597;
    let t20511 = t2195 * t6148;
    let t20539 = 0.82757551241431752271e-2 * t5 * t7 * t4888 * t511 * t1603 * t2116;
    let t20541 = t512 * t6101 * t57;
    (t20484, t20499, t20511, t20539, t20541)
}
