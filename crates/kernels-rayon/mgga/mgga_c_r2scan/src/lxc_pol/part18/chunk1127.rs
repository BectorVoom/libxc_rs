//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1127/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1127(t40840: f64, t3366: f64, t8355: f64, t12005: f64, t1338: f64, t3678: f64, t6755: f64, t1348: f64, t6767: f64, t11561: f64, t11863: f64, t11864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40841 = 2.0_f64 / 3.0_f64 * t40840;
    let t40844 = t8355 * t3366;
    let t40845 = 2.0_f64 / 3.0_f64 * t40844;
    let t41028 = t1338 * t12005;
    let t41039 = t6755 * t3678;
    let t41042 = t1348 * t12005;
    let t41047 = t6767 * t3678;
    let t41104 = 5.0_f64 / 8.0_f64 * t11561;
    let t41105 = 2.0_f64 * t11863;
    let t41106 = 2.0_f64 * t11864;
    (t40841, t40845, t41028, t41039, t41042, t41047, t41104, t41105, t41106)
}
