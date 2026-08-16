//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1060/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1060(t167: f64, t7704: f64, t14554: f64, t1003: f64, t4781: f64, t26686: f64, t4977: f64, t7691: f64, t5329: f64, t4972: f64, t7709: f64, t1094: f64, t1748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27903 = t7704 * t167;
    let t27904 = t14554 * t27903;
    let t27910 = t4781 * t1003;
    let t27911 = t26686 * t27910;
    let t27914 = t7691 * t4977;
    let t27915 = t5329 * t27914;
    let t27918 = t7709 * t4972;
    let t27919 = t5329 * t27918;
    let t27924 = t1748 * t1094;
    (t27903, t27904, t27910, t27911, t27914, t27915, t27918, t27919, t27924)
}
