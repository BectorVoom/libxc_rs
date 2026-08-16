//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1009;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta265<F: Float>(t138: F, t9675: F, t9674: F, t4075: F, t556: F, t786: F, t1444: F, t2434: F, t123: F, t3915: F, t1359: F, t9292: F, t1363: F, t9288: F, t1362: F, t3911: F, t3920: F, t2237: F, t240: F, t550: F, t816: F, t1379: F, t2689: F, t3952: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9677, t9680, t9687, t9691) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1009::<F>(t138, t9675, t9674, t4075, t556, t786, t1444, t2434, t123, t3915, t1359, t9292);
        let (t9694, t9695, t9707, t9711, t9712) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1010::<F>(t1363, t9288, t1362, t3911, t3920, t2237, t240, t550, t816, t1379, t2689, t3952);
    (t9677, t9680, t9687, t9691, t9694, t9695, t9707, t9711, t9712)
}
