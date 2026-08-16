//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1091/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1091(t2887: f64, t5616: f64, t68: f64, t5612: f64, t771: f64, t178: f64, t299: f64, t301: f64, t4902: f64, t5604: f64, t775: f64, t2065: f64, t2082: f64) -> (f64, f64, f64, f64, f64) {
    let t17890 = t2887 * t68 * t5616;
    let t17897 = t771 * t5612;
    let t17902 = 0.14820648238345094262e-3_f64 * t299 * t178 * t4902 * t301;
    let t17903 = t5604 * t775;
    let t17905 = t2082 * t2065;
    (t17890, t17897, t17902, t17903, t17905)
}
