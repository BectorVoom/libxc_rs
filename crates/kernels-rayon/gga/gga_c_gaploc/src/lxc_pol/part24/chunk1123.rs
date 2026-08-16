//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1123/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1123(t21665: f64, t9760: f64, t21451: f64, t5539: f64, t9647: f64, t21784: f64, t2554: f64, t7064: f64, t7276: f64, t3240: f64, t7211: f64, t2549: f64, t9630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29480 = 0.1281754371690370714e-2_f64 * t21665 * t9760;
    let t29483 = 0.2563508743380741428e-2_f64 * t9647 * t5539 * t21451;
    let t29486 = 0.1281754371690370714e-2_f64 * t9647 * t5539 * t21784;
    let t29489 = 0.1281754371690370714e-2_f64 * t7064 * t7276 * t2554;
    let t29492 = 0.64087718584518535698e-3_f64 * t7211 * t3240;
    let t29494 = 0.1281754371690370714e-2_f64 * t2549 * t9630;
    (t29480, t29483, t29486, t29489, t29492, t29494)
}
