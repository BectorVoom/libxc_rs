//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2396;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta630<F: Float>(t40769: F, t810: F, t820: F, t849: F, t9948: F, t857: F, t10722: F, t2479: F, t14832: F, t2430: F, t2475: F, t2661: F, t775: F, t2699: F, t2729: F, t2732: F, t235: F, t4503: F, t2453: F, t10728: F, t9794: F, t10886: F, t40236: F, t808: F, t123: F, t125: F, t2452: F, t40633: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40771, t40781, t40782, t40784, t40789) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2396::<F>(t40769, t810, t820, t849, t9948, t857, t10722, t2479, t14832, t2430, t2475, t2661, t775);
        let (t40791, t40792, t40798, t40799, t40801, t40804, t40810) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2397::<F>(t2699, t2729, t2732, t235, t4503, t2453, t10728, t9794, t10886, t40236, t808, t123, t125, t2452, t40633, t810);
    (t40771, t40781, t40782, t40784, t40789, t40791, t40792, t40798, t40799, t40801, t40804, t40810)
}
