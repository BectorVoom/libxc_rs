//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta156<F: Float>(t1150: F, t3385: F, t3384: F, t406: F, t409: F, t1134: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t1132: F) -> (F, F, F, F, F, F, F, F) {
        let (t3386, t3388, t3390, t3391, t3392, t3394, t3399, t3400) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk811::<F>(t1150, t3385, t3384, t406, t409, t1134, t3356, t3358, t3365, t3370, t3374, t1132);
    (t3386, t3388, t3390, t3391, t3392, t3394, t3399, t3400)
}
