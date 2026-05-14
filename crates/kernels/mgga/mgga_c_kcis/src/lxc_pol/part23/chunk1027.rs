//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1027/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1027<F: Float>(t1591: F, t17960: F, t4413: F, t6136: F, t12857: F, t2093: F, t51622: F, t5737: F, t286: F, t69: F, t1610: F, t167: F, t2104: F, t4457: F, t26391: F, t26399: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54581 = t17960 * t1591;
    let t54605 = t6136 * t4413;
    let t54624 = t2093 * t12857;
    let t59319 = t51622 * t5737;
    let t61287 = t69 * t286;
    let t61402 = t1610 * t167;
    let t61664 = t2104 * t4457;
    let t91769 = 18.0 * t26391;
    let t91772 = 6.0 * t26399;
    (t54581, t54605, t54624, t59319, t61287, t61402, t61664, t91769, t91772)
}
