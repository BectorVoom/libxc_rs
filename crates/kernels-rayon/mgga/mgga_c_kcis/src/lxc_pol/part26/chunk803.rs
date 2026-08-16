//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 803/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk803(t11580: f64, t455: f64, t11407: f64, t11481: f64, t127: f64, t1392: f64, t368: f64, t456: f64, t518: f64, t531: f64, t10338: f64, t1474: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11581 = t455 * t11580;
    let t11608 = 0.93011851851851851854e0_f64 * t11407;
    let t11609 = 0.36514074074074074075e0_f64 * t11481;
    let t11632 = t127 * t368 * t1392;
    let t11633 = t456 * t518;
    let t11634 = t11633 * t531;
    let t11640 = t10338 * t1474;
    (t11581, t11608, t11609, t11632, t11633, t11634, t11640)
}
