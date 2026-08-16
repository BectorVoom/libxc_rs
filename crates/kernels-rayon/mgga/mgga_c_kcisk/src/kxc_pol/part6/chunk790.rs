//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 790/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk790(t12261: f64, t2327: f64, t535: f64, t2097: f64, t3722: f64, t2285: f64, t4460: f64, t4435: f64, t3696: f64, t2318: f64, t4416: f64, t2306: f64, t4346: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21674 = t12261 * t2327;
    let t21675 = t535 * t21674;
    let t21748 = t2097 * t3722;
    let t21764 = t2285 * t4460;
    let t21869 = t2285 * t4435;
    let t21872 = t2097 * t3696;
    let t21902 = t2318 * t4416;
    let t21969 = t2306 * t4346;
    (t21675, t21748, t21764, t21869, t21872, t21902, t21969)
}
