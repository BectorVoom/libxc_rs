//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 548/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk548(t1123: f64, t2861: f64, t984: f64, t987: f64, t983: f64, t990: f64, t110: f64, t292: f64, t285: f64, t24: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2862 = t2861 * t1123;
    let t2870 = t984 * t987;
    let t2872 = t983 * t990;
    let t2877 = t110 * t292;
    let t2879 = t285 * t2877 / 432.0_f64;
    let t2880 = t24 * t992;
    (t2862, t2870, t2872, t2877, t2879, t2880)
}
