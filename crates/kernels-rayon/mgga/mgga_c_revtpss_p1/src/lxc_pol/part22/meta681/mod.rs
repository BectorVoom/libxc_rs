//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta681 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta681(t19680: f64, t70: f64, t18281: f64, t36: f64, t5826: f64, t627: f64, t1486: f64, t4181: f64, t4187: f64, t1470: f64, t4217: f64, t1494: f64, t21686: f64, t21687: f64, t21690: f64, t4182: f64, t5820: f64, t5827: f64, t5830: f64, t641: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21695, t21698, t21699, t21702, t21707, t21710, t21713, t21720) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2666(t19680, t70, t18281, t36, t5826, t627, t1486, t4181, t4187, t1470, t4217, t1494, t21686, t21687, t21690, t4182, t5820, t5827, t5830, t641, t85);
    (t21695, t21698, t21699, t21702, t21707, t21710, t21713, t21720)
}
