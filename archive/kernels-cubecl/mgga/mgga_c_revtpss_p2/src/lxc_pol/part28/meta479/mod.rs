//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1818;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1819;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta479<F: Float>(t3167: F, t7120: F, t1033: F, t3173: F, t7122: F, t2269: F, t343: F, t136: F, t1007: F, t7106: F, t1968: F, t3080: F, t7105: F, t800: F, t1017: F, t1028: F, t1047: F, t25490: F, t25495: F, t25498: F, t25500: F, t25505: F, t25509: F, t25512: F, t25517: F, t25522: F, t3097: F, t3130: F, t3136: F, t3157: F, t3164: F, t3208: F, t3220: F, t348: F, t7117: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25525, t25526) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1818::<F>(t3167, t7120, t1033);
        let (t25529, t25531, t25532, t25535, t25538, t25539) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1819::<F>(t3173, t7122, t2269, t343, t136, t1007, t7106, t1968, t3080, t7105, t800);
        let t25542 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1820::<F>(t1017, t1028, t1047, t25490, t25495, t25498, t25500, t25505, t25509, t25512, t25517, t25522, t25526, t25529, t25532, t25535, t25538, t25539, t3097, t3130, t3136, t3157, t3164, t3208, t3220, t348, t7117, t7122);
    (t25525, t25526, t25529, t25531, t25532, t25535, t25538, t25539, t25542)
}
