//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1410;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1411;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta373<F: Float>(t15008: F, t689: F, t213: F, t4469: F, t1580: F, t2440: F, t2439: F, t1569: F, t2453: F, t2458: F, t4321: F, t887: F, t4470: F, t786: F, t789: F, t4534: F, t779: F, t2435: F, t4322: F, t1596: F, t2873: F, t1614: F, t2942: F, t1606: F, t4580: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15010, t15011, t15015, t15018, t15045) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1410::<F>(t15008, t689, t213, t4469, t1580, t2440, t2439, t1569, t2453, t2458, t4321, t887);
        let (t15047, t15050, t15062, t15063, t15101, t15104) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1411::<F>(t15045, t689, t4470, t786, t789, t4534, t779, t2435, t4322, t1596, t2873, t1614, t2942);
        let (t15123, t15125) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1412::<F>(t1606, t2439, t4580, t689);
    (t15010, t15011, t15015, t15018, t15047, t15050, t15062, t15063, t15101, t15104, t15123, t15125)
}
