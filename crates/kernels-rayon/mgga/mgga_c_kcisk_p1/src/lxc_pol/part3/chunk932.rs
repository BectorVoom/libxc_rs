//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 932/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk932(t338: f64, t1323: f64, t164: f64, t1309: f64, t3966: f64, t3984: f64, t25: f64, t3989: f64, t13125: f64, t1320: f64, t1310: f64, t122: f64, t4000: f64) -> (f64, f64, f64, f64, f64) {
    let t400 = 0.0_f64 < t338;
    let t13804 = t164 * t1323;
    let t13805 = t1309 * t13804;
    let t13807 = t3966 * t3984;
    let t13809 = t25 * t3989;
    let t13810 = t1309 * t13809;
    let t13815 = piecewise3(t400, t13125, -t13125);
    let t13816 = t1320 * t13815;
    let t13817 = t1310 * t13816;
    let t13820 = t4000 * t122;
    (t13805, t13807, t13810, t13817, t13820)
}
