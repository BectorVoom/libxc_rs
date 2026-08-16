//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta381<F: Float>(t15827: F, t4837: F, t1659: F, t3105: F, t1062: F, t4797: F, t1660: F, t3201: F, t1058: F, t4798: F, t15127: F, t15125: F) -> (F, F, F, F, F, F, F) {
        let (t15829, t15830, t15850, t15862, t15865, t15874, t15875) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1418::<F>(t15827, t4837, t1659, t3105, t1062, t4797, t1660, t3201, t1058, t4798, t15127, t15125);
    (t15829, t15830, t15850, t15862, t15865, t15874, t15875)
}
