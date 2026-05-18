//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1187/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1187<F: Float>(t11881: F, t7925: F, t27428: F, t4142: F, t1014: F, t27479: F, t2237: F, t54162: F, t7915: F, t7900: F, t27348: F, t27410: F) -> (F, F, F, F, F, F, F) {
    let t94472 = t11881 * t7925;
    let t94474 = t4142 * t27428;
    let t94483 = t1014 * t27479;
    let t94489 = t2237 * t54162 * t7915;
    let t94491 = t54162 * t7900;
    let t94492 = t2237 * t94491;
    let t94494 = t27410 * t27348;
    (t94472, t94474, t94483, t94489, t94491, t94492, t94494)
}
