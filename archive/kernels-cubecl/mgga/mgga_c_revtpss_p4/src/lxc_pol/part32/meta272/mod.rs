//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1149;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1150;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1151;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta272<F: Float>(t3: F, t8113: F, param_d: F, t1518: F, t7553: F, t117: F, t7983: F, t1916: F, t1918: F, t2113: F, t2115: F, t572: F, t573: F, t587: F, t65: F, t197: F, t532: F, t1450: F, t2106: F, t143: F, t2580: F, t130: F, t2566: F, t700: F, t2584: F, t121: F, t131: F, t141: F, t22: F, t2456: F, t624: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t8114, t8118) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1149::<F>(t3, t8113, param_d);
        let (t8124, t8127, t8130, t8779) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1150::<F>(t1518, t7553, t117, t7983, t1916, t1918, t2113, t2115, t572, t573, t8118, t587, t65);
        let (t8995, t9069, t9275, t9278) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1151::<F>(t197, t532, t1450, t2106, t143, t2580, t130, t2566, t700, t2584);
        let (t9283, t9285) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1152::<F>(t121, t131, t141, t22, t2456, t624);
    (t8114, t8118, t8124, t8127, t8130, t8779, t8995, t9069, t9275, t9278, t9283, t9285)
}
