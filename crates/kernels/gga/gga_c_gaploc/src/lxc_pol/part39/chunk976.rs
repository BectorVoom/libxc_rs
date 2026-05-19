//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 976/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk976<F: Float>(t12807: F, t6313: F, t6305: F, t2268: F, t41784: F, t6320: F, t39774: F, t39778: F, t12826: F, t12840: F, t3137: F, t7930: F) -> (F, F, F, F, F, F, F, F) {
    let t42797 = F::cast_from(0.22764005308540919679e0_f64) * t6313 * t12807;
    let t42799 = F::cast_from(0.17073003981405689759e0_f64) * t6305 * t12807;
    let t42802 = F::cast_from(0.17073003981405689759e0_f64) * t2268 * t6320 * t41784;
    let t42803 = F::cast_from(0.23712505529730124666e-2_f64) * t39774;
    let t42804 = F::cast_from(0.47425011059460249332e-2_f64) * t39778;
    let t42806 = F::cast_from(0.45528010617081839357e0_f64) * t6313 * t12826;
    let t42808 = F::cast_from(0.85365019907028448797e-1_f64) * t6305 * t12840;
    let t42811 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t7930 * t3137;
    (t42797, t42799, t42802, t42803, t42804, t42806, t42808, t42811)
}
