//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta598<F: Float>(t7642: F, t96873: F, t26948: F, t487: F, t8945: F, t26936: F, t3736: F, t7635: F, t3566: F, t1203: F, t1294: F, t1209: F) -> (F, F, F, F, F, F, F) {
        let (t97034, t97041, t97050, t97065, t97066, t97067, t97078) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2059::<F>(t7642, t96873, t26948, t487, t8945, t26936, t3736, t7635, t3566, t1203, t1294, t1209);
    (t97034, t97041, t97050, t97065, t97066, t97067, t97078)
}
