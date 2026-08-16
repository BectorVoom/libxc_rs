//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta521<F: Float>(t378: F, t4743: F, t1678: F, t989: F, t15654: F, t1086: F, t1089: F, t15920: F, t16076: F, t12073: F, t1651: F, t1082: F, t16152: F) -> (F, F, F, F, F, F, F, F) {
        let (t16362, t16371, t16374, t16381, t16390, t16393, t16396, t16399) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2158::<F>(t378, t4743, t1678, t989, t15654, t1086, t1089, t15920, t16076, t12073, t1651, t1082, t16152);
    (t16362, t16371, t16374, t16381, t16390, t16393, t16396, t16399)
}
