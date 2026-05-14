//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1289/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1289<F: Float>(t27949: F, t546: F, t2182: F, t979: F, t7630: F, t29775: F, t538: F, t6155: F, t29951: F, t1616: F, t2892: F, t5095: F, t785: F, t2294: F, t6583: F, t8774: F) -> (F, F, F, F, F, F) {
    let t30364 = t546 * t27949;
    let t30370 = t2182 * t979;
    let t30371 = t30370 * t7630;
    let t30374 = t6155 * t538 * t29775;
    let t30377 = t6155 * t538 * t29951;
    let t30381 = t5095 * t785 * t1616 * t2892;
    let t30394 = t6583 * t2294 * t8774;
    (t30364, t30371, t30374, t30377, t30381, t30394)
}
