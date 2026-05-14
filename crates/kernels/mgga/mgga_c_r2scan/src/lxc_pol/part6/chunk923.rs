//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 923/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk923<F: Float>(t44: F, t1569: F, t6133: F, t2148: F, t6535: F, t1550: F, t277: F, t113: F, t2147: F, t2155: F, t5169: F, t1212: F, t1219: F, t473: F, t4905: F, t4913: F, t99: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t6536 = t6133 * t1569;
    let t6537 = t2148 * t6536;
    let t6538 = t6535 * t6537;
    let t6540 = t277 * t1550;
    let t6541 = t6540 * t113;
    let t6542 = t2148 * t6541;
    let t6543 = t2147 * t6542;
    let t6545 = t2155 * t5169;
    let t6556 = piecewise3(t45, 0.0, -10.0 / 27.0 * t1212 * t4905 + 10.0 / 3.0 * t473 * t1219 + 5.0 / 3.0 * t99 * t4913);
    (t6536, t6537, t6538, t6540, t6541, t6542, t6543, t6545, t6556)
}
