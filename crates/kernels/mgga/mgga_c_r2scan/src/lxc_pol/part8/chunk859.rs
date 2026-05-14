//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 859/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk859<F: Float>(t2747: F, t468: F, t1411: F, t963: F, t1385: F, t486: F, t95: F, t5052: F, t910: F, t1541: F, t2526: F, t3270: F, t792: F, t1561: F, t983: F, t2847: F, t498: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7155 = t2747 * t468;
    let t7156 = 0.11696447245269292414e1 * t7155;
    let t7157 = t963 * t1411;
    let t7159 = t963 * t1385;
    let t7175 = t486 * t95;
    let t7180 = t5052 * t910;
    let t7184 = t1541 * t2526;
    let t7206 = t3270 * t792;
    let t7217 = t1561 * t983;
    let t7218 = t7217 * t792;
    let t7221 = t498 * t2847;
    (t7155, t7156, t7157, t7159, t7175, t7180, t7184, t7206, t7217, t7218, t7221)
}
