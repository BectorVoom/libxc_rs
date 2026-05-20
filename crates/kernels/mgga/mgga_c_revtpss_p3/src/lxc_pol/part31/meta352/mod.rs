//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta352<F: Float>(t3930: F, t5661: F, t5665: F, t9976: F, t1412: F, t1882: F, t3938: F, t3992: F, t2661: F, t1399: F, t5608: F, t5651: F) -> (F, F, F, F, F, F, F, F) {
        let (t14042, t14043, t14045, t14046, t14049, t14050, t14053, t14054) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1366::<F>(t3930, t5661, t5665, t9976, t1412, t1882, t3938, t3992, t2661, t1399, t5608, t5651);
    (t14042, t14043, t14045, t14046, t14049, t14050, t14053, t14054)
}
