//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 936/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk936(t28699: f64, t716: f64, t740: f64, t748: f64, t2567: f64, t9089: f64, t5284: f64, t2586: f64, t9050: f64, t5315: f64, t29274: f64, t4971: f64) -> (f64, f64, f64, f64) {
    let t29584 = t28699 * t716;
    let t29585 = t29584 * t740;
    let t29586 = t29585 * t748;
    let t29588 = t2567 * t9089;
    let t29589 = t5284 * t29588;
    let t29590 = t2586 * t9050;
    let t29591 = t5315 * t29590;
    let t29593 = t4971 * t29274;
    (t29586, t29589, t29591, t29593)
}
