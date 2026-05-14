//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 765/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk765<F: Float>(t39849: F, t12803: F, t29874: F, t31586: F, t4261: F, t9074: F, t1063: F, t2854: F, t29969: F, t6320: F, t12767: F, t6313: F, t2268: F, t2756: F, t3152: F, t39866: F) -> (F, F, F, F, F, F, F) {
    let t42845 = 0.142275033178380748e-1 * t39849;
    let t42846 = t29874 * t12803;
    let t42847 = 0.47425011059460249332e-2 * t42846;
    let t42849 = t9074 * t4261 * t31586;
    let t42850 = 0.47425011059460249332e-2 * t42849;
    let t42857 = 0.17073003981405689759e0 * t1063 * t6320 * t2854 * t29969;
    let t42863 = 0.7588001769513639893e-1 * t6313 * t12767;
    let t42866 = 0.28455006635676149599e-1 * t2268 * t3152 * t2756;
    let t42867 = 0.47425011059460249332e-2 * t39866;
    (t42845, t42847, t42850, t42857, t42863, t42866, t42867)
}
