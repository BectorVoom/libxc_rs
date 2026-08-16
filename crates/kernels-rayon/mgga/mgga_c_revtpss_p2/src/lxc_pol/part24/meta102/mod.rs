//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta102 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk585;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk586;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta102(t3431: f64, t408: f64, t421: f64, t3356: f64, t1159: f64, t431: f64, t426: f64, t3413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3432, t3433) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk585(t3431, t408);
        let (t3434, t3435) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk586(t421);
        let (t3439, t3450, t3451, t3452, t3459, t3466, t3475) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk587(t3356, t1159, t431, t426, t3413);
    (t3432, t3433, t3434, t3435, t3439, t3450, t3451, t3452, t3459, t3466, t3475)
}
