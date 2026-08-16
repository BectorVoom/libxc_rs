//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1036/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1036(t31000: f64, t3484: f64, t3482: f64, t13129: f64, t2075: f64, t7757: f64, t2083: f64, t3539: f64, t7740: f64, t13138: f64, t7877: f64, t2191: f64, t3544: f64) -> (f64, f64, f64, f64, f64) {
    let t31001 = t3484 * t31000;
    let t31002 = t3482 * t31001;
    let t31009 = t13129 * t2075 * t7757;
    let t31013 = t3539 * t7740 * t2083;
    let t31017 = t13138 * t2075 * t7877;
    let t31021 = t3544 * t7740 * t2191;
    (t31002, t31009, t31013, t31017, t31021)
}
