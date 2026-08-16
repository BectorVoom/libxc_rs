//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta877 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3043;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta877<F: Float>(t14939: F, t233: F, t689: F, t869: F, t10069: F, t14588: F, t10518: F, t14606: F, t231: F, t2782: F, t2783: F, t51380: F, t10073: F, t14504: F, t10547: F, t14568: F, t50560: F, t2797: F, t18632: F, t836: F, t10529: F, t14602: F, t2482: F, t2811: F, t4423: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t51505, t51507, t51512, t51519) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3043::<F>(t14939, t233, t689, t869, t10069, t14588, t10518, t14606, t231, t2782, t2783, t51380);
        let (t51521, t51523, t51527, t51529, t51531, t51535) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3044::<F>(t10073, t14504, t10547, t14568, t231, t50560, t2782, t2797, t18632, t836, t10529, t14602, t2482, t2811, t4423);
    (t51505, t51507, t51512, t51519, t51521, t51523, t51527, t51529, t51531, t51535)
}
