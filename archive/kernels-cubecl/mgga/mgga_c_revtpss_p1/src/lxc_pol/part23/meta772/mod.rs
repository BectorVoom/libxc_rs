//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta772 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2575;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2576;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta772<F: Float>(t17395: F, t3746: F, t12268: F, t29054: F, t12898: F, t1786: F, t17202: F, t372: F, t44546: F, t5340: F, t5342: F, t11772: F, t17394: F, t3717: F, t12865: F, t17400: F, t1222: F, t1781: F, t2438: F, t12854: F, t21013: F, t12808: F, t3698: F, t5047: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57571, t57606, t57615, t57621, t57636, t57659) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2575::<F>(t17395, t3746, t12268, t29054, t12898, t1786, t17202, t372, t44546, t5340, t5342, t11772, t17394);
        let (t57660, t57663, t57687, t57707, t57710, t57726) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2576::<F>(t3717, t57659, t12865, t17400, t1222, t1781, t2438, t12854, t21013, t12808, t3698, t5047, t697);
    (t57571, t57606, t57615, t57621, t57636, t57659, t57660, t57663, t57687, t57707, t57710, t57726)
}
