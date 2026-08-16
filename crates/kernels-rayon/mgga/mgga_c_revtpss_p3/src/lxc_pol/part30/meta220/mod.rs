//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1028;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1029;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta220(t1015: f64, t4186: f64, t1012: f64, t3147: f64, t72: f64, t3088: f64, t3299: f64, t1668: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t4886, t4887, t4890) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1028(t1015, t4186, t1012, t3147, t72);
        let t4891 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1029(t3088, t4890);
        let (t4892, t4893) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1030(t3299, t4891, t1668, t3153);
    (t4886, t4887, t4890, t4891, t4892, t4893)
}
