//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta401<F: Float>(t1247: F, t17544: F, t3707: F, t5292: F, t12268: F, t3617: F, t3708: F, t5265: F, t1260: F, t5326: F, t3704: F, t5274: F) -> (F, F, F, F, F, F) {
        let (t17546, t17547, t17550, t17556, t17569, t17593) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1452::<F>(t1247, t17544, t3707, t5292, t12268, t3617, t3708, t5265, t1260, t5326, t3704, t5274);
    (t17546, t17547, t17550, t17556, t17569, t17593)
}
