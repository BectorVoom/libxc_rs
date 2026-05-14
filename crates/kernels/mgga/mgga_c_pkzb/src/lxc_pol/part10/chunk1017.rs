//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1017/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1017<F: Float>(t8429: F, t8436: F, t406: F, t3207: F, t8380: F, t2387: F, t394: F) -> (F, F, F, F, F) {
    let t8437 = t8429 * t8436;
    let t8438 = t406 * t8437;
    let t8441 = t8380 * t3207;
    let t8442 = t406 * t8441;
    let t8445 = t2387 * t394;
    (t8437, t8438, t8441, t8442, t8445)
}
