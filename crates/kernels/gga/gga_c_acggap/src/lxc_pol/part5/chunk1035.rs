//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1035/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1035<F: Float>(t1008: F, t6116: F, t6121: F, t6106: F, t1016: F, t1795: F) -> (F, F, F, F) {
    let t20737 = t1008 * t6116;
    let t20739 = t1008 * t6121;
    let t20753 = t1008 * t6106;
    let t20764 = t1016 * t1795;
    (t20737, t20739, t20753, t20764)
}
