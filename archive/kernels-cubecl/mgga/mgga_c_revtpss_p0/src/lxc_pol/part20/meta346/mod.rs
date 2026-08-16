//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1273;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta346<F: Float>(t12712: F, t3629: F, t12702: F, t5330: F, t12744: F, t1214: F, t5341: F, t1250: F, t140: F, t3698: F, t1012: F, t13026: F, t12268: F, t3617: F, t2258: F, t3628: F, t3367: F, t471: F, t2251: F, t17350: F, t3767: F, t1121: F, t1248: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17354, t17426, t17429, t17454, t17459, t17471, t17475) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1273::<F>(t12712, t3629, t12702, t5330, t12744, t1214, t5341, t1250, t140, t3698, t1012, t13026);
        let (t17550, t17638, t17644, t17654, t17656) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1274::<F>(t12268, t3617, t2258, t3628, t3367, t471, t2251, t17350, t3767, t1121, t1248, t606);
    (t17354, t17426, t17429, t17454, t17459, t17471, t17475, t17550, t17638, t17644, t17654, t17656)
}
