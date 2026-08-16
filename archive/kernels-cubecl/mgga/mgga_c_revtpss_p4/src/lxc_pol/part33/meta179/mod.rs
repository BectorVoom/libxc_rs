//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta179<F: Float>(t366: F, t4797: F, t1065: F, t2857: F, t4181: F, t1042: F, t2852: F, t3181: F, t1592: F, t3109: F, t247: F, t1063: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4798, t4801, t4802, t4803, t4806, t4807, t4808, t4817, t4818) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk863::<F>(t366, t4797, t1065, t2857, t4181, t1042, t2852, t3181, t1592, t3109, t247, t1063);
    (t4798, t4801, t4802, t4803, t4806, t4807, t4808, t4817, t4818)
}
