//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1038;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1039;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta173(t4893: f64, t4900: f64, t3117: f64, t1043: f64, t1651: f64, t1045: f64, t357: f64, t999: f64, t4781: f64, t1012: f64, t1014: f64, t4579: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4901, t4902, t4905, t4906, t4907, t4910) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1038(t4893, t4900, t3117, t1043, t1651, t1045, t357, t999);
        let (t4911, t4912, t4915) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1039(t4781, t4910, t3117, t1012, t1014);
        let (t4916, t4919) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1040(t4579, t4915, t1012, t3252);
    (t4901, t4902, t4905, t4906, t4907, t4910, t4911, t4912, t4915, t4916, t4919)
}
