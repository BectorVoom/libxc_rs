//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1143/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1143<F: Float>(t1861: F, t7614: F, t1998: F, t5826: F, t1165: F, t5651: F, t604: F, t8463: F, t1815: F, t406: F, t1181: F, t599: F, t7413: F) -> (F, F, F, F, F) {
    let t39784 = t7614 * t1861;
    let t39786 = t1998 * t5826;
    let t39790 = t8463 * t1165 * t604 * t5651;
    let t39794 = t1815 * t406;
    let t39797 = t7413 * t1181 * t599 * t39794;
    (t39784, t39786, t39790, t39794, t39797)
}
