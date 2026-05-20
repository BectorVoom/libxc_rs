//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2410;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta637<F: Float>(t11010: F, t689: F, t779: F, t2769: F, t786: F, t861: F, t10997: F, t11007: F, t252: F, t11009: F, t123: F, t676: F, t11006: F, t256: F, t225: F, t2782: F, t2828: F, t886: F, t2441: F, t39515: F, t10504: F, t138: F, t9302: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41063, t41066, t41067, t41070, t41073) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2410::<F>(t11010, t689, t779, t2769, t786, t861, t10997, t11007, t252, t11009, t123, t676);
        let (t41078, t41092, t41095, t41098) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2411::<F>(t11006, t256, t225, t252, t2769, t2782, t2828, t886, t2441, t39515, t10504, t138, t9302);
    (t41063, t41066, t41067, t41070, t41073, t41078, t41092, t41095, t41098)
}
