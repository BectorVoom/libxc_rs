//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta824 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2677;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta824<F: Float>(t11773: F, t4954: F, t1011: F, t6284: F, t697: F, t19900: F, t3241: F, t19477: F, t3153: F, t15905: F, t56017: F, t55899: F, t15700: F, t19992: F, t53405: F, t16226: F, t19997: F, t11710: F, t19777: F, t3091: F, t19644: F, t140: F, t19916: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t66542, t66547, t66551, t66565, t66621, t66624) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2677::<F>(t11773, t4954, t1011, t6284, t697, t19900, t3241, t19477, t3153, t15905, t56017, t55899);
        let (t66644, t66647, t66655, t66660, t66686) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2678::<F>(t15700, t19992, t53405, t16226, t19997, t11710, t19777, t3091, t19644, t1011, t140, t19916);
    (t66542, t66547, t66551, t66565, t66621, t66624, t66644, t66647, t66655, t66660, t66686)
}
