//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta619<F: Float>(t1045: F, t19497: F, t3117: F, t1043: F, t11631: F, t19450: F, t4894: F, t19501: F, t4910: F, t11274: F, t11277: F, t11789: F, t11875: F, t15684: F, t15906: F, t16081: F, t19731: F, t19738: F, t19741: F, t3091: F, t3115: F, t4896: F, t4902: F, t6308: F, t6312: F, t6339: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19744, t19745, t19748, t19749, t19750, t19753, t19754, t19757, t19758, t19763) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2527::<F>(t1045, t19497, t3117, t1043, t11631, t19450, t4894, t19501, t4910, t11274, t11277, t11789, t11875, t15684, t15906, t16081, t19731, t19738, t19741, t3091, t3115, t4896, t4902, t6308, t6312, t6339);
    (t19744, t19745, t19748, t19749, t19750, t19753, t19754, t19757, t19758, t19763)
}
