//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta470(t26506: f64, t7064: f64, t2061: f64, t2722: f64, t25416: f64, t2723: f64, t231: f64, t7076: f64, t136: f64, t2066: f64, t2457: f64, t25299: f64, t25365: f64, t7407: f64, t1956: f64, t213: f64, t25383: f64, t257: f64, t26437: f64, t26439: f64, t26441: f64, t26448: f64, t26475: f64, t26483: f64, t26486: f64, t26489: f64, t26493: f64, t26498: f64, t26500: f64, t26502: f64, t7067: f64, t7070: f64, t7415: f64, t7424: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26508, t26511, t26515, t26518, t26519) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1732(t26506, t7064, t2061, t2722, t25416, t2723, t231, t7076, t136, t2066, t2457);
        let (t26521, t26522, t26524) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1733(t25299, t26519, t25365, t7407, t1956, t213, t25383, t257, t26437, t26439, t26441, t26448, t26475, t26483, t26486, t26489, t26493, t26498, t26500, t26502, t26508, t26511, t26515, t7067, t7070, t7415, t7424);
    (t26508, t26511, t26515, t26518, t26519, t26521, t26522, t26524)
}
