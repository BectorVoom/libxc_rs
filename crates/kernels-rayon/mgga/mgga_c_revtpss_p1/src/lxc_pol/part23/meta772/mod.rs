//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta772 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2575;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2576;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta772(t17395: f64, t3746: f64, t12268: f64, t29054: f64, t12898: f64, t1786: f64, t17202: f64, t372: f64, t44546: f64, t5340: f64, t5342: f64, t11772: f64, t17394: f64, t3717: f64, t12865: f64, t17400: f64, t1222: f64, t1781: f64, t2438: f64, t12854: f64, t21013: f64, t12808: f64, t3698: f64, t5047: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57571, t57606, t57615, t57621, t57636, t57659) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2575(t17395, t3746, t12268, t29054, t12898, t1786, t17202, t372, t44546, t5340, t5342, t11772, t17394);
        let (t57660, t57663, t57687, t57707, t57710, t57726) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2576(t3717, t57659, t12865, t17400, t1222, t1781, t2438, t12854, t21013, t12808, t3698, t5047, t697);
    (t57571, t57606, t57615, t57621, t57636, t57659, t57660, t57663, t57687, t57707, t57710, t57726)
}
