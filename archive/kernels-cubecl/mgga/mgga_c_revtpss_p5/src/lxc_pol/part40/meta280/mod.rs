//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1025;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta280<F: Float>(t10019: F, t4101: F, t555: F, t5744: F, t786: F, t3923: F, t675: F, t268: F, t4003: F, t2435: F, t4093: F, t4083: F, t9303: F, t4066: F, t545: F, t869: F, t689: F, t2777: F, t4092: F, t2439: F, t2782: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10020, t10022, t10024, t10027, t10032, t10035) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1025::<F>(t10019, t4101, t555, t5744, t786, t3923, t675, t268, t4003, t2435, t4093, t4083, t9303);
        let (t10041, t10044, t10059, t10062) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1026::<F>(t4066, t545, t869, t689, t2777, t4092, t2439, t3923, t555, t4003, t5744, t2782);
    (t10020, t10022, t10024, t10027, t10032, t10035, t10041, t10044, t10059, t10062)
}
