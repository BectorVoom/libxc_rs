//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta529<F: Float>(t1448: F, t1907: F, t28197: F, t28196: F, t7316: F, t7898: F, t13426: F, t1936: F, t18227: F, t4248: F, t7002: F, t27123: F) -> (F, F, F, F, F, F, F, F) {
        let (t28198, t28199, t28201, t28202, t28212, t28214, t28216, t28218) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1944::<F>(t1448, t1907, t28197, t28196, t7316, t7898, t13426, t1936, t18227, t4248, t7002, t27123);
    (t28198, t28199, t28201, t28202, t28212, t28214, t28216, t28218)
}
