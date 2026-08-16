//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1895;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta633(t1361: f64, t22690: f64, t6330: f64, t80840: f64, t22792: f64, t6347: f64, t26318: f64, t7708: f64, t91351: f64, t19844: f64, t6916: f64, t22804: f64, t28077: f64, t22779: f64, t28067: f64, t19924: f64, t26288: f64, t19994: f64, t19919: f64, t221: f64, t91194: f64, t26284: f64, t91198: f64, t20000: f64, t91361: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97427, t97431, t97435, t97437, t97439) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1895(t1361, t22690, t6330, t80840, t22792, t6347, t26318, t7708, t91351, t19844, t6916, t22804, t28077);
        let (t97444, t97447, t97450, t97453, t97456, t97459, t97461) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1896(t22779, t28067, t1361, t19924, t26288, t19994, t19919, t221, t91194, t26284, t91198, t20000, t91361);
    (t97427, t97431, t97435, t97437, t97439, t97444, t97447, t97450, t97453, t97456, t97459, t97461)
}
