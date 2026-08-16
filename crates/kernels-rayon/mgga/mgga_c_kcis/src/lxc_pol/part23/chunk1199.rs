//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1199/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1199(t1628: f64, t27671: f64, t27733: f64, t26656: f64, t13093: f64, t2167: f64, t4527: f64, t7671: f64, t93826: f64, t1655: f64, t26654: f64, t28311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t95235 = t27671 * t1628;
    let t95271 = 2.0_f64 * t27733;
    let t95275 = 4.0_f64 * t26656;
    let t97548 = t13093 * t2167;
    let t97561 = 2.0_f64 * t4527 * t7671;
    let t97584 = 2.0_f64 * t93826;
    let t97601 = t1655 * t26654;
    let t97622 = t28311 / 8.0_f64;
    (t95235, t95271, t95275, t97548, t97561, t97584, t97601, t97622)
}
