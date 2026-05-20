//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta738 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2514;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta738<F: Float>(t50977: F, t40672: F, t828: F, t14819: F, t40517: F, t14741: F, t2710: F, t2713: F, t10744: F, t14861: F, t808: F, t40791: F, t4442: F, t14742: F, t2689: F, t243: F, t9794: F, t10760: F, t14495: F, t14587: F, t40799: F, t4372: F, t9789: F, t40627: F, t50451: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50978, t51014, t51042, t51055, t51059, t51060) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2514::<F>(t50977, t40672, t828, t14819, t40517, t14741, t2710, t2713, t10744, t14861, t808, t40791, t4442);
        let (t51061, t51074, t51079, t51081, t51083, t51086) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2515::<F>(t51060, t14742, t2689, t243, t9794, t10760, t14495, t14587, t40799, t4372, t9789, t40627, t50451);
    (t50978, t51014, t51042, t51055, t51059, t51061, t51074, t51079, t51081, t51083, t51086)
}
