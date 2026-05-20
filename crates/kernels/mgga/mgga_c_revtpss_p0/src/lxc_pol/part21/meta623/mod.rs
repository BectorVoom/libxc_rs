//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2382;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2383;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta623<F: Float>(t10752: F, t10905: F, t2783: F, t9801: F, t10745: F, t2735: F, t4503: F, t10728: F, t808: F, t10680: F, t2710: F, t2713: F, t10732: F, t10744: F, t10674: F, t2693: F, t9732: F, t14917: F, t2475: F, t2661: F, t2662: F, t836: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40511, t40517, t40518, t40523, t40526) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2382::<F>(t10752, t10905, t2783, t9801, t10745, t2735, t4503, t10728, t808, t10680, t2710, t2713);
        let (t40529, t40532, t40535, t40549) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2383::<F>(t10732, t10744, t808, t10674, t2710, t2713, t2693, t9732, t14917, t2475, t2661, t2662, t836);
    (t40511, t40517, t40518, t40523, t40526, t40529, t40532, t40535, t40549)
}
