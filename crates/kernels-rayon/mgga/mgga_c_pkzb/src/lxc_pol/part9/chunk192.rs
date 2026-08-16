//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 192/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk192(t51: f64, t600: f64, t164: f64, t592: f64, t499: f64, t66: f64, t168: f64, t167: f64, t180: f64, t589: f64) -> (f64, f64, f64, f64, f64) {
    let t601 = t51 * t600;
    let t603 = t592 * t601 * t164;
    let t607 = 1.0_f64 / t66 / t499;
    let t608 = t168 * t607;
    let t611 = 0.10003937560882938627e-2_f64 * t167 * t608 * t180;
    let t612 = t167 * t589;
    (t603, t607, t608, t611, t612)
}
