//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 795/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk795<F: Float>(t2531: F, t2567: F, t360: F, t2573: F, t8820: F, t2551: F, t2572: F, t495: F, t2124: F, t2550: F, t8837: F, t8001: F, t921: F, t2562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9135 = t2567 * t2531;
    let t9136 = t360 * t9135;
    let t9139 = t8820 * t2573;
    let t9140 = t360 * t9139;
    let t9143 = t8820 * t2551;
    let t9144 = t360 * t9143;
    let t9147 = t2572 * t2531;
    let t9148 = t360 * t9147;
    let t9151 = t8820 * t495;
    let t9152 = t360 * t9151;
    let t9156 = t2124 * t2550 * t2531;
    let t9160 = t2124 * t8837 * t495;
    let t9165 = t8001 * t921;
    let t9166 = t360 * t9165;
    let t9169 = t2562 * t2531;
    let t9170 = t360 * t9169;
    (t9135, t9136, t9139, t9140, t9143, t9144, t9147, t9148, t9151, t9152, t9156, t9160, t9165, t9166, t9169, t9170)
}
