//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta726 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2566;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2567;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta726(t9575: f64, t9860: f64, t3869: f64, t39538: f64, t39427: f64, t39535: f64, t2496: f64, t9551: f64, t4038: f64, t9372: f64, t1317: f64, t9428: f64, t3853: f64, t3857: f64, t820: f64, t843: f64, t9991: f64, t9997: f64, t1386: f64, t2237: f64, t2482: f64, t4021: f64, t235: f64, t46475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47135, t47138, t47140, t47142, t47145, t47147, t47149) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2566(t9575, t9860, t3869, t39538, t39427, t39535, t2496, t9551, t4038, t9372, t1317, t9428);
        let (t47152, t47195, t47198, t47199, t47201) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2567(t3853, t3857, t820, t843, t9991, t9997, t1386, t2237, t2482, t4021, t235, t46475);
    (t47135, t47138, t47140, t47142, t47145, t47147, t47149, t47152, t47195, t47198, t47199, t47201)
}
