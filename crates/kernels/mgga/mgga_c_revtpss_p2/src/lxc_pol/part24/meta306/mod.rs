//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta306<F: Float>(t5916: F, t625: F, t10227: F, t5895: F, t10241: F, t5907: F, t6785: F, t9335: F, t6792: F, t9350: F, t1450: F, t6922: F) -> (F, F, F, F, F, F) {
        let (t21827, t21835, t21860, t21906, t21918, t21937) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1091::<F>(t5916, t625, t10227, t5895, t10241, t5907, t6785, t9335, t6792, t9350, t1450, t6922);
    (t21827, t21835, t21860, t21906, t21918, t21937)
}
