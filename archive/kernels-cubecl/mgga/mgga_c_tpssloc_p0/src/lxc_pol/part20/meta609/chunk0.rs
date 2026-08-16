//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2194/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2194<F: Float>(t11677: F, t11904: F, t11702: F, t3536: F, t11709: F, t11745: F, t11651: F, t11734: F, t1174: F, t3556: F, t698: F, t11844: F, t135: F) -> (F, F, F, F, F, F) {
    let t45162 = t11904 * t11677;
    let t45167 = t3536 * t11702;
    let t45169 = t11709 * t11745;
    let t45171 = t11734 * t11651;
    let t45178 = t1174 * t698 * t3556;
    let t45181 = t1174 * t135 * t11844;
    (t45162, t45167, t45169, t45171, t45178, t45181)
}
