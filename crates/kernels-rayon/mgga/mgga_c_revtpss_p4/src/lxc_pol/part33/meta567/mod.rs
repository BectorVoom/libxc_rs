//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1970;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta567(t1774: f64, t8197: f64, t7637: f64, t2148: f64, t6695: f64, t1287: f64, t6622: f64, t7660: f64, t26907: f64, t3769: f64, t6628: f64, t1769: f64, t1208: f64, t487: f64, t1828: f64, t8190: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30866, t30867, t30870, t30874, t30878, t30881) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1970(t1774, t8197, t7637, t2148, t6695, t1287, t6622, t7660, t26907, t3769, t6628, t1769);
        let (t30882, t30883, t30886) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1971(t1208, t30881, t487, t1828, t8190);
    (t30866, t30867, t30870, t30874, t30878, t30881, t30882, t30883, t30886)
}
