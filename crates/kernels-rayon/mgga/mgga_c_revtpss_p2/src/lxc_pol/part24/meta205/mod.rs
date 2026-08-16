//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta205 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta205(t243: f64, t816: f64, t9707: f64, t813: f64, t247: f64, t9949: f64, t237: f64, t236: f64, t9646: f64, t9721: f64, t268: f64, t207: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t10673, t10687, t10688, t10689, t10692, t10696) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk939(t243, t816, t9707, t813, t247, t9949, t237, t236, t9646, t9721, t268, t207, t242);
    (t10673, t10687, t10688, t10689, t10692, t10696)
}
