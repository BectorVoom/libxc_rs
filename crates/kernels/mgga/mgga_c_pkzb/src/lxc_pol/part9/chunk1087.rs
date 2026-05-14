//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1087/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1087<F: Float>(t2860: F, t5742: F, t1977: F, t237: F, t1108: F, t20638: F, t5500: F, t1991: F, t7560: F, t1083: F, t5776: F, t5585: F, t5578: F, t2870: F, t5754: F, t1987: F, t7223: F) -> (F, F, F, F, F, F, F, F) {
    let t20670 = 0.51947577317044391277e2 * t2860 * t5742;
    let t20671 = t237 * t1977;
    let t20674 = 0.10526802520742363173e2 * t20671 * t1108 * t20638;
    let t20676 = 0.10389515463408878255e3 * t2860 * t5500;
    let t20678 = 0.35089341735807877242e1 * t7560 * t1991;
    let t20683 = t5776 * t1083;
    let t20685 = 0.2894756309764656312e3 * t20683 * t5585;
    let t20687 = 0.5848223622634646207e0 * t2860 * t5578;
    let t20693 = 0.17544670867903938621e1 * t5754 * t2870;
    let t20695 = 0.51947577317044391277e2 * t1987 * t7223;
    (t20670, t20674, t20676, t20678, t20685, t20687, t20693, t20695)
}
