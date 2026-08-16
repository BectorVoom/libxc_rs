//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2396;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta630(t40769: f64, t810: f64, t820: f64, t849: f64, t9948: f64, t857: f64, t10722: f64, t2479: f64, t14832: f64, t2430: f64, t2475: f64, t2661: f64, t775: f64, t2699: f64, t2729: f64, t2732: f64, t235: f64, t4503: f64, t2453: f64, t10728: f64, t9794: f64, t10886: f64, t40236: f64, t808: f64, t123: f64, t125: f64, t2452: f64, t40633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40771, t40781, t40782, t40784, t40789) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2396(t40769, t810, t820, t849, t9948, t857, t10722, t2479, t14832, t2430, t2475, t2661, t775);
        let (t40791, t40792, t40798, t40799, t40801, t40804, t40810) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2397(t2699, t2729, t2732, t235, t4503, t2453, t10728, t9794, t10886, t40236, t808, t123, t125, t2452, t40633, t810);
    (t40771, t40781, t40782, t40784, t40789, t40791, t40792, t40798, t40799, t40801, t40804, t40810)
}
