//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1397;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta441<F: Float>(t3869: F, t39538: F, t39427: F, t39535: F, t3853: F, t3857: F, t73: F, t9940: F, t820: F, t843: F, t9991: F, t1386: F, t2237: F, t2482: F, t235: F, t46475: F, t239: F, t4000: F, t596: F, t72: F, t245: F, t136: F, t4010: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47138, t47140, t47142, t47152, t47171, t47194, t47198) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1397::<F>(t3869, t39538, t39427, t39535, t3853, t3857, t73, t9940, t820, t843, t9991, t1386, t2237, t2482);
        let (t47203, t47215, t47248, t47273) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1398::<F>(t235, t46475, t239, t820, t2482, t4000, t596, t72, t9940, t245, t136, t4010);
    (t47138, t47140, t47142, t47152, t47171, t47194, t47198, t47203, t47215, t47248, t47273)
}
