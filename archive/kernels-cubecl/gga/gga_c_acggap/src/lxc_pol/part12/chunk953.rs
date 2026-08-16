//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 953/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk953<F: Float>(t31539: F, t368: F, t7457: F, t7458: F, t7310: F, t7386: F, t7637: F, t7753: F, t7447: F, t7816: F, t1967: F, t7689: F) -> (F, F, F, F, F) {
    let t31601 = t7457 * t7458 * t368 * t31539;
    let t31603 = t7310 * t7386;
    let t31605 = t7637 * t7753;
    let t31607 = t7447 * t7816;
    let t31609 = t1967 * t7689;
    (t31601, t31603, t31605, t31607, t31609)
}
