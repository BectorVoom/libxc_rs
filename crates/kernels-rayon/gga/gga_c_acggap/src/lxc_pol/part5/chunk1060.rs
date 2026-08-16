//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1060/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1060(t13083: f64, t3360: f64, t4284: f64, t1434: f64, t3700: f64, t1165: f64, t15407: f64, t3456: f64, t540: f64, t12727: f64, t1470: f64, t1137: f64, t4594: f64) -> (f64, f64, f64, f64, f64) {
    let t18686 = t3360 * t13083 * t4284;
    let t18690 = t3700 * t1434;
    let t18702 = t3456 * t1165 * t540 * t15407;
    let t18704 = t12727 * t1470;
    let t18719 = t1137 * t4594;
    (t18686, t18690, t18702, t18704, t18719)
}
