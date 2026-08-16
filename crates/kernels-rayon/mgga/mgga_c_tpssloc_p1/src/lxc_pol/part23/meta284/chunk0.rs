//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 980/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk980(t20861: f64, t819: f64, t820: f64, t20853: f64, t232: f64, t5527: f64, t4181: f64, t9646: f64, t16839: f64, t2645: f64, t5591: f64, t1484: f64, t2632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20963 = t819 * t820 * t20861;
    let t20969 = t819 * t820 * t20853;
    let t20972 = t232 * t5527;
    let t20974 = t9646 * t4181 * t20972;
    let t20978 = t2645 * t16839 * t5591;
    let t20981 = t2632 * t1484;
    (t20963, t20969, t20972, t20974, t20978, t20981)
}
