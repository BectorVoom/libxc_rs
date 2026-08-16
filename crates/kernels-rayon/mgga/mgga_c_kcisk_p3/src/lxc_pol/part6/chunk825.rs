//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 825/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk825(t1248: f64, t13603: f64, t7736: f64, t3979: f64, t7740: f64, t7744: f64, t4126: f64, t7993: f64, t45: f64, t7970: f64, t4083: f64, t7959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26176 = t1248 * t13603 * t7736;
    let t26179 = t1248 * t3979 * t7740;
    let t26198 = t1248 * t3979 * t7744;
    let t26302 = t4126 * t7993;
    let t26341 = t45 * t7970;
    let t26344 = t7959 * t4083;
    (t26176, t26179, t26198, t26302, t26341, t26344)
}
