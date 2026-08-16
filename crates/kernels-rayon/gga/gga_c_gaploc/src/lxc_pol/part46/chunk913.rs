//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 913/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk913(t12807: f64, t6313: f64, t6305: f64, t2268: f64, t41784: f64, t6320: f64, t39774: f64, t39778: f64, t12826: f64, t12840: f64, t3137: f64, t7930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42797 = 0.22764005308540919679e0_f64 * t6313 * t12807;
    let t42799 = 0.17073003981405689759e0_f64 * t6305 * t12807;
    let t42802 = 0.17073003981405689759e0_f64 * t2268 * t6320 * t41784;
    let t42803 = 0.23712505529730124666e-2_f64 * t39774;
    let t42804 = 0.47425011059460249332e-2_f64 * t39778;
    let t42806 = 0.45528010617081839357e0_f64 * t6313 * t12826;
    let t42808 = 0.85365019907028448797e-1_f64 * t6305 * t12840;
    let t42811 = 0.85365019907028448797e-1_f64 * t2268 * t7930 * t3137;
    (t42797, t42799, t42802, t42803, t42804, t42806, t42808, t42811)
}
