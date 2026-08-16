//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2179;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta654<F: Float>(t2251: F, t3953: F, t1437: F, t2303: F, t72: F, t4021: F, t641: F, t645: F, t7445: F, t12619: F, t71: F, t1433: F, t2307: F, t12719: F, t79: F, t1410: F, t9228: F, t2235: F, t3961: F, t3967: F, t26117: F, t6534: F, t1268: F, t86604: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90205, t90227, t90232, t90247, t90257, t90297) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2179::<F>(t2251, t3953, t1437, t2303, t72, t4021, t641, t645, t7445, t12619, t71, t1433, t2307);
        let (t90334, t90337, t90340, t90343, t90355, t90361) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2180::<F>(t12719, t72, t79, t1410, t9228, t2235, t3961, t3967, t26117, t6534, t1268, t86604);
    (t90205, t90227, t90232, t90247, t90257, t90297, t90334, t90337, t90340, t90343, t90355, t90361)
}
