//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1070/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1070<F: Float>(t1599: F, t18163: F, t12844: F, t6155: F, t4439: F, t3970: F, t617: F, t5441: F, t12140: F, t5427: F, t12217: F, t16905: F) -> (F, F, F, F, F, F) {
    let t18164 = t1599 * t18163;
    let t18168 = t12844 * t6155;
    let t18170 = t4439 * t18168 / F::cast_from(864.0_f64);
    let t18171 = t3970 * t617;
    let t18172 = t18171 * t5441;
    let t18174 = t4439 * t18172 / F::cast_from(432.0_f64);
    let t18175 = t12140 * t617;
    let t18176 = t18175 * t5427;
    let t18178 = t4439 * t18176 / F::cast_from(648.0_f64);
    let t18183 = t12217 * t617;
    let t18187 = t16905 * t617;
    (t18164, t18170, t18174, t18178, t18183, t18187)
}
