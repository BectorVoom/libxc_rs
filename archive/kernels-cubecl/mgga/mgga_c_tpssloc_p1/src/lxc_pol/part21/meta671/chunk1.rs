//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2475/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2475<F: Float>(t11779: F, t820: F, t11677: F, t11907: F, t11904: F, t1174: F, t3556: F, t698: F, t11153: F, t1176: F, t11881: F, t45113: F) -> (F, F, F, F, F, F) {
    let t45128 = t820 * t11779;
    let t45134 = t11907 * t11677;
    let t45162 = t11904 * t11677;
    let t45178 = t1174 * t698 * t3556;
    let t45192 = t1176 * t11153;
    let t45197 = t11881 * t45113;
    (t45128, t45134, t45162, t45178, t45192, t45197)
}
