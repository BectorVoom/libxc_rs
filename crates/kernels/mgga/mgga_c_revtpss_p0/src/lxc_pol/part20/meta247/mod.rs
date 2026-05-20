//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1074;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1075;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1076;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta247<F: Float>(t11202: F, t996: F, t3325: F, t999: F, t1079: F, t3043: F, t378: F, t3042: F, t993: F, t1000: F, t1076: F, t1097: F, t11123: F, t11128: F, t11174: F, t11178: F, t11184: F, t11187: F, t11190: F, t11195: F, t11201: F, t3047: F, t3052: F, t3060: F, t3076: F, t3261: F, t3326: F, t989: F, t995: F, t1071: F, t3056: F, t988: F, t2258: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11203, t11207, t11210, t11213) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1074::<F>(t11202, t996, t3325, t999, t1079, t3043, t378, t3042, t993);
        let (t11214, t11217) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1075::<F>(t11213, t378, t1000, t1076, t1097, t11123, t11128, t11174, t11178, t11184, t11187, t11190, t11195, t11201, t11203, t11207, t11210, t3047, t3052, t3060, t3076, t3261, t3326, t989, t995);
        let (t11220, t11223) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1076::<F>(t1071, t989, t3056, t988);
        let (t11224, t11231) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1077::<F>(t11223, t378, t2258, t606);
    (t11203, t11207, t11210, t11213, t11214, t11217, t11220, t11223, t11224, t11231)
}
