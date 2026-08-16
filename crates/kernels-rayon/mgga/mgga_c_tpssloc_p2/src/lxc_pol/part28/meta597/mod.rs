//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1895;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta597(t12652: f64, t605: f64, t12661: f64, t4017: f64, t645: f64, t72: f64, t1433: f64, t2241: f64, t12568: f64, t608: f64, t2251: f64, t3953: f64, t1437: f64, t2303: f64, t4021: f64, t641: f64, t7445: f64, t12619: f64, t71: f64, t2307: f64, t12719: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90153, t90160, t90177, t90196, t90202, t90205) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1895(t12652, t605, t12661, t4017, t645, t72, t1433, t2241, t12568, t608, t2251, t3953);
        let (t90227, t90232, t90247, t90257, t90297, t90334) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1896(t1437, t2303, t72, t4021, t641, t645, t7445, t12619, t71, t1433, t2307, t12719, t79);
    (t90153, t90160, t90177, t90196, t90202, t90205, t90227, t90232, t90247, t90257, t90297, t90334)
}
