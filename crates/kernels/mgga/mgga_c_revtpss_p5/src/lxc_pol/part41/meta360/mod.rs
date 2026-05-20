//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1174;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta360<F: Float>(t1222: F, t17472: F, t1012: F, t13026: F, t1263: F, t5245: F, t1234: F, t5390: F, t3704: F, t5293: F, t3172: F, t5286: F, t1247: F, t3707: F, t5292: F, t12268: F, t3617: F, t3708: F, t5265: F, t1260: F, t5326: F, t5274: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17474, t17475, t17500, t17505, t17509, t17544) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1174::<F>(t1222, t17472, t1012, t13026, t1263, t5245, t1234, t5390, t3704, t5293, t3172, t5286);
        let (t17546, t17547, t17550, t17556, t17569, t17593) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1175::<F>(t1247, t17544, t3707, t5292, t12268, t3617, t3708, t5265, t1260, t5326, t3704, t5274);
    (t17474, t17475, t17500, t17505, t17509, t17546, t17547, t17550, t17556, t17569, t17593)
}
