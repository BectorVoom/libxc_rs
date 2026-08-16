//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 852/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk852(t5520: f64, t5522: f64, t7357: f64, t7359: f64, t9148: f64, t9163: f64, t665: f64, t3528: f64, t5547: f64, t667: f64, t2759: f64, t2765: f64) -> (f64, f64, f64, f64, f64) {
    let t9164 = -t5520 + 4.0_f64 / 9.0_f64 * t5522 + 8.0_f64 / 9.0_f64 * t7357 - t7359 - t9148 / 3.0_f64 + t9163;
    let t9165 = t665 * t9164;
    let t9171 = t5547 * t3528;
    let t9172 = t9171 * t667;
    let t9174 = t2765 * t2759;
    (t9164, t9165, t9171, t9172, t9174)
}
