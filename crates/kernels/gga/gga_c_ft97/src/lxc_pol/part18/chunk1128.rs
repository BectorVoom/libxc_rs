//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1128/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1128<F: Float>(t1369: F, t23621: F, t376: F, t23625: F, t23604: F, t40280: F, t91: F, t1882: F, t23618: F, t23916: F, t375: F, t89: F, t1900: F, t2086: F, t6: F) -> (F, F, F, F, F, F, F) {
    let t95245 = t1369 * t376 * t23621;
    let t95252 = t1369 * t376 * t23625;
    let t95254 = t1369 * t376 * t23604;
    let t95262 = t91 * t40280;
    let t95269 = t1882 * t23618;
    let t95289 = t89 * t375 * t23916;
    let t95292 = t91 * t2086 * t6 * t1900;
    (t95245, t95252, t95254, t95262, t95269, t95289, t95292)
}
