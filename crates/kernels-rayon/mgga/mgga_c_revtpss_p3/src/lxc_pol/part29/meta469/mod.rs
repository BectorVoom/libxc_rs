//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1730;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta469(t25387: f64, t26485: f64, t2061: f64, t2771: f64, t25317: f64, t7398: f64, t886: f64, t7071: f64, t2062: f64, t867: f64, t786: f64, t2467: f64, t25431: f64, t26482: f64, t225: f64, t26473: f64, t2470: f64, t7406: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26486, t26488, t26489, t26492, t26493, t26496, t26497) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1730(t25387, t26485, t2061, t2771, t25317, t7398, t886, t7071, t2062, t867, t786);
        let (t26498, t26500, t26502, t26506) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1731(t2467, t26497, t25431, t26482, t225, t26473, t2470, t7406);
    (t26486, t26488, t26489, t26492, t26493, t26496, t26497, t26498, t26500, t26502, t26506)
}
