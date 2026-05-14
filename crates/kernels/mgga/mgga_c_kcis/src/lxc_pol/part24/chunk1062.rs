//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1062/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1062<F: Float>(t283: F, t33822: F, t990: F, t1009: F, t14407: F, t2811: F, t44756: F, t27873: F, t9386: F, t27796: F, t2822: F, t27765: F, t2861: F, t27769: F, t27815: F, t7703: F, t9938: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t95535 = t33822 * t283 * t990;
    let t95552 = t14407 * t1009;
    let t95557 = t44756 * t2811;
    let t95571 = t9386 * t27873;
    let t95572 = 0.3684876543209876543e-2 * t95571;
    let t95581 = t2822 * t27796;
    let t95585 = t2861 * t27765;
    let t95586 = 0.66327777777777777776e-2 * t95585;
    let t95587 = t2861 * t27769;
    let t95605 = 0.15445601851851851852e-3 * t7703 * t9938 * t27815;
    (t95535, t95552, t95557, t95571, t95572, t95581, t95585, t95586, t95587, t95605)
}
