//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2473/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2473<F: Float>(t1213: F, t1216: F, t248: F, t45017: F, t11716: F, t44833: F, t44834: F, t3503: F, t1174: F, t1197: F, t2402: F, t3584: F, t676: F) -> (F, F, F, F, F) {
    let t45020 = t1213 * t248 * t45017 * t1216;
    let t45030 = t44833 * t11716 * t44834;
    let t45037 = t44833 * t3503 * t44834;
    let t45044 = t1174 * t2402 * t1197;
    let t45046 = t676 * t3584;
    (t45020, t45030, t45037, t45044, t45046)
}
