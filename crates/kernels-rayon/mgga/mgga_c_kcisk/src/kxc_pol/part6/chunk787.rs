//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 787/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk787(t1387: f64, t2059: f64, t14100: f64, t2181: f64, t2192: f64, t3812: f64, t3831: f64, t2110: f64, t3929: f64, t2240: f64, t4169: f64, t19848: f64, t492: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20781 = t1387 * t2059;
    let t20783 = t14100 * t2059;
    let t20796 = t1387 * t2181;
    let t20798 = t3812 * t2192;
    let t20820 = t3831 * t2059;
    let t20886 = t2110 * t3929;
    let t20922 = t2240 * t4169;
    let t21066 = t19848 * t492;
    (t20781, t20783, t20796, t20798, t20820, t20886, t20922, t21066)
}
