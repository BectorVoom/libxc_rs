//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk845;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta173(t2847: f64, t2848: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t291: f64, t1596: f64, t914: f64, t936: f64, t1610: f64, t2869: f64, t934: f64, t2874: f64, t1600: f64, t2880: f64, t918: f64, t2884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4587, t4589, t4590, t4592, t4594) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk845(t2847, t2848, t4571, t4576, t4581, t4585, t291, t1596, t914, t936, t1610, t2869);
        let (t4595, t4597, t4598, t4599, t4606) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk846(t1610, t934, t2874, t1600, t2880, t918, t2848, t2884, t4571, t4576, t4581, t4585);
    (t4587, t4589, t4590, t4592, t4594, t4595, t4597, t4598, t4599, t4606)
}
