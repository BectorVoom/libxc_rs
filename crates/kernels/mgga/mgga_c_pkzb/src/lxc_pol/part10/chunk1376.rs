//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1376/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1376<F: Float>(t2243: F, t27494: F, t1185: F, t22357: F, t3153: F, t8028: F, t237: F, t9973: F, t900: F, t18520: F, t3806: F, t8288: F, t898: F, t18609: F, t3769: F, t27479: F, t27481: F, t27484: F, t27488: F, t27491: F, t27493: F) -> (F, F, F, F, F, F, F) {
    let t27496 = 0.16081979498692535067e2 * t27494 * t2243;
    let t27498 = 2.0 * t22357 * t1185;
    let t27500 = 0.46785788981077169656e1 * t8028 * t3153;
    let t27501 = t237 * t9973;
    let t27503 = 0.11696447245269292414e1 * t27501 * t900;
    let t27507 = 0.12304822629859687989e5 * t898 * t18520 * t3806 * t8288;
    let t27509 = 0.16081979498692535067e2 * t18609 * t3769;
    let t27510 = -t27479 + t27481 - t27484 + t27488 - t27491 + t27493 + t27496 + t27498 + t27500 - t27503 + t27507 + t27509;
    (t27496, t27498, t27500, t27503, t27507, t27509, t27510)
}
