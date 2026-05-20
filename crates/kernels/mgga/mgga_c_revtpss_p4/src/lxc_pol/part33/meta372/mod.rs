//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1408;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta372<F: Float>(t4533: F, t72: F, t686: F, t2465: F, t1569: F, t867: F, t786: F, t2467: F, t122: F, t4480: F, t2466: F, t10995: F, t11044: F, t4481: F, t2435: F, t4477: F, t136: F, t1579: F, t2457: F, t10504: F, t2471: F, t4325: F, t1580: F, t2444: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14983, t14985, t14987, t14989, t14991, t14992) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1408::<F>(t4533, t72, t686, t2465, t1569, t867, t786, t2467, t122, t4480, t2466, t10995);
        let (t14995, t14998, t15003, t15004, t15006, t15008) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1409::<F>(t11044, t4481, t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444);
    (t14983, t14985, t14987, t14989, t14991, t14992, t14995, t14998, t15003, t15004, t15006, t15008)
}
