//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1184/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1184(t283: f64, t33822: f64, t990: f64, t1009: f64, t14407: f64, t2811: f64, t44756: f64, t27873: f64, t9386: f64, t27796: f64, t2822: f64, t27765: f64, t2861: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95535 = t33822 * t283 * t990;
    let t95552 = t14407 * t1009;
    let t95557 = t44756 * t2811;
    let t95571 = t9386 * t27873;
    let t95572 = 0.3684876543209876543e-2_f64 * t95571;
    let t95581 = t2822 * t27796;
    let t95585 = t2861 * t27765;
    (t95535, t95552, t95557, t95571, t95572, t95581, t95585)
}
