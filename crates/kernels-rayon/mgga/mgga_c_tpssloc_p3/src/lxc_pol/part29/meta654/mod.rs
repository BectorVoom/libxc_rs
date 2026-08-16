//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2179;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta654(t2251: f64, t3953: f64, t1437: f64, t2303: f64, t72: f64, t4021: f64, t641: f64, t645: f64, t7445: f64, t12619: f64, t71: f64, t1433: f64, t2307: f64, t12719: f64, t79: f64, t1410: f64, t9228: f64, t2235: f64, t3961: f64, t3967: f64, t26117: f64, t6534: f64, t1268: f64, t86604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90205, t90227, t90232, t90247, t90257, t90297) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2179(t2251, t3953, t1437, t2303, t72, t4021, t641, t645, t7445, t12619, t71, t1433, t2307);
        let (t90334, t90337, t90340, t90343, t90355, t90361) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2180(t12719, t72, t79, t1410, t9228, t2235, t3961, t3967, t26117, t6534, t1268, t86604);
    (t90205, t90227, t90232, t90247, t90257, t90297, t90334, t90337, t90340, t90343, t90355, t90361)
}
