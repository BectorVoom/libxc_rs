//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1125/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1125<F: Float>(t1345: F, t30148: F, t30154: F, t7842: F, t34569: F, t8465: F, t5281: F, t7561: F, t1992: F, t30692: F, t5720: F, t30364: F, t5147: F) -> (F, F, F, F, F) {
    let t35995 = t30154 * t7842 * t30148 * t1345;
    let t35997 = t34569 * t8465;
    let t35999 = t7561 * t5281;
    let t36004 = t30692 * t7842 * t1992 * t5720;
    let t36006 = t30364 * t5147;
    (t35995, t35997, t35999, t36004, t36006)
}
