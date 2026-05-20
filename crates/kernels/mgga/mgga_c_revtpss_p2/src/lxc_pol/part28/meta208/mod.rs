//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk995;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta208<F: Float>(t2847: F, t2848: F, t4571: F, t4576: F, t4581: F, t4585: F, t291: F, t1596: F, t914: F, t936: F, t1610: F, t2869: F, t934: F, t2874: F, t1600: F, t2880: F, t918: F, t2884: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4587, t4589, t4590, t4592, t4594) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk995::<F>(t2847, t2848, t4571, t4576, t4581, t4585, t291, t1596, t914, t936, t1610, t2869);
        let (t4595, t4597, t4598, t4599, t4606) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk996::<F>(t1610, t934, t2874, t1600, t2880, t918, t2848, t2884, t4571, t4576, t4581, t4585);
    (t4587, t4589, t4590, t4592, t4594, t4595, t4597, t4598, t4599, t4606)
}
