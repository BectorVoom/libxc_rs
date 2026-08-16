//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1005 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3436;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1005(t15573: f64, t4719: f64, t11524: f64, t19133: f64, t981: f64, t15526: f64, t19134: f64, t3022: f64, t15266: f64, t52894: f64, t63597: f64, t19021: f64, t3011: f64, t4733: f64, t19049: f64, t3034: f64, t19045: f64, t300: f64, t983: f64, t63940: f64, t63943: f64, t64327: f64, t64329: f64, t64488: f64, t64491: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64493, t64496, t64498, t64500, t64503, t64504) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3436(t15573, t4719, t11524, t19133, t981, t15526, t19134, t3022, t15266, t52894, t63597, t19021, t3011);
        let (t64507, t64509, t64512, t64513) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3437(t4733, t64504, t981, t19049, t3034, t19045, t300, t983, t63940, t63943, t64327, t64329, t64488, t64491, t64493, t64496, t64498, t64500, t64503);
    (t64493, t64496, t64498, t64500, t64503, t64507, t64509, t64512, t64513)
}
