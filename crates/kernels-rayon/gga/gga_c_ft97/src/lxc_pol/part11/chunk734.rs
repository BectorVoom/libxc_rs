//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 734/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk734(t1775: f64, t2489: f64, t2508: f64, t458: f64, t192: f64, t743: f64, t9692: f64, t462: f64, t92: f64, t9931: f64, t9933: f64, t9935: f64, t9936: f64, t9939: f64, t9944: f64, t9949: f64, t9955: f64, t9958: f64) -> (f64, f64) {
    let t9960 = t1775 * t2489;
    let t9962 = t458 * t2508;
    let t9965 = t192 * t743 * t9692;
    let t9967 = t462 * t9931 + t9933 - t9935 - 4.0_f64 / 3.0_f64 * t9936 - t462 * t9939 / 3.0_f64 - 6.0_f64 * t92 * t9944 + 6.0_f64 * t462 * t9949 - 10.0_f64 / 27.0_f64 * t462 * t9955 + t9958 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t9960 - 2.0_f64 * t9962 - t92 * t9965;
    (t9965, t9967)
}
