//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 972/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk972<F: Float>(t18171: F, t5441: F, t4439: F, t12140: F, t617: F, t5427: F, t12217: F, t16905: F, t1928: F, t610: F, t990: F, t4426: F, t6141: F, t25: F, t494: F, t6178: F) -> (F, F, F, F, F, F, F) {
    let t18172 = t18171 * t5441;
    let t18174 = t4439 * t18172 / 432.0;
    let t18175 = t12140 * t617;
    let t18176 = t18175 * t5427;
    let t18178 = t4439 * t18176 / 648.0;
    let t18183 = t12217 * t617;
    let t18187 = t16905 * t617;
    let t18192 = t610 * t1928 * t990;
    let t18205 = t6141 * t4426 / 324.0;
    let t18210 = t25 * t494;
    let t18211 = t18210 * t6178;
    (t18174, t18178, t18183, t18187, t18192, t18205, t18211)
}
