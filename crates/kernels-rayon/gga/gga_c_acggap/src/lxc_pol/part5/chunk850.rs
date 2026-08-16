//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 850/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk850(t11934: f64, t265: f64, t272: f64, t286: f64, t11787: f64, t13: f64, t2803: f64, t758: f64, t775: f64, t2955: f64, t883: f64, t685: f64) -> (f64, f64, f64, f64) {
    let t11938 = 0.5848223622634646207e0_f64 * t286 * t265 * t11934 * t272;
    let t11944 = 0.62071215503128080361e4_f64 * t13 / t775 / t758 * t11787 * t2803;
    let t11945 = t883 * t2955;
    let t11947 = t685 * t685;
    (t11938, t11944, t11945, t11947)
}
