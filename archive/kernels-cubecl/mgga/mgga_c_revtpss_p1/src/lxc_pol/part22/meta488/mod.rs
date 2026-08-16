//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta488<F: Float>(t3169: F, t4820: F, t1015: F, t13312: F, t1012: F, t16096: F, t4573: F, t11703: F, t3188: F, t4817: F, t1011: F, t11268: F, t11714: F, t11967: F, t11972: F, t11980: F, t11989: F, t12007: F, t12010: F, t16095: F, t1671: F, t1675: F) -> (F, F, F, F, F, F) {
        let (t16121, t16122, t16127, t16128, t16134, t16136) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2211::<F>(t3169, t4820, t1015, t13312, t1012, t16096, t4573, t11703, t3188, t4817, t1011, t11268, t11714, t11967, t11972, t11980, t11989, t12007, t12010, t16095, t1671, t1675);
    (t16121, t16122, t16127, t16128, t16134, t16136)
}
