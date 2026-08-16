//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1096/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1096(t19822: f64, t496: f64, t6825: f64, t184: f64, t5418: f64, t16388: f64, t2583: f64, t149: f64, t5224: f64, t63: f64, t1041: f64, t17095: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19823 = 0.51947577317044391276e2_f64 * t19822;
    let t19824 = t496 * t6825;
    let t19825 = 12.0_f64 * t19824;
    let t19873 = t184 * t5418;
    let t19909 = t16388 * t2583;
    let t19910 = 35.0_f64 / 24.0_f64 * t19909;
    let t19932 = t149 * t5224 * t63;
    let t19947 = t17095 * t1041;
    (t19823, t19825, t19873, t19910, t19932, t19947)
}
