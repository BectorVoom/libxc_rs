//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 763/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk763<F: Float>(t12807: F, t6313: F, t6305: F, t2268: F, t41784: F, t6320: F, t39774: F, t39778: F, t12826: F, t12840: F, t3137: F, t7930: F, t2765: F, t9152: F, t39791: F, t39794: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t42797 = 0.22764005308540919679e0 * t6313 * t12807;
    let t42799 = 0.17073003981405689759e0 * t6305 * t12807;
    let t42802 = 0.17073003981405689759e0 * t2268 * t6320 * t41784;
    let t42803 = 0.23712505529730124666e-2 * t39774;
    let t42804 = 0.47425011059460249332e-2 * t39778;
    let t42806 = 0.45528010617081839357e0 * t6313 * t12826;
    let t42808 = 0.85365019907028448797e-1 * t6305 * t12840;
    let t42811 = 0.85365019907028448797e-1 * t2268 * t7930 * t3137;
    let t42814 = 0.85365019907028448797e-1 * t2268 * t2765 * t9152;
    let t42815 = 0.23712505529730124666e-2 * t39791;
    let t42816 = 0.23712505529730124666e-2 * t39794;
    (t42797, t42799, t42802, t42803, t42804, t42806, t42808, t42811, t42814, t42815, t42816)
}
