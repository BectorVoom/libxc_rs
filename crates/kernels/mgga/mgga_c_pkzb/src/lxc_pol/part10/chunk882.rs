//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 882/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk882<F: Float>(t6087: F, t6174: F, t2295: F, t877: F, t2256: F, t858: F, t369: F, t6230: F) -> (F, F, F, F, F) {
    let t6249 = 0.16068111111111111111e1 * t6087;
    let t6256 = 0.46308888888888888888e0 * t6174;
    let t6266 = t877 * t2295;
    let t6272 = t858 * t2256;
    let t6282 = t369 * t6230;
    (t6249, t6256, t6266, t6272, t6282)
}
