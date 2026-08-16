//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 765/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk765(t1046: f64, t3054: f64, t308: f64, t9758: f64, t1042: f64, t2943: f64, t9725: f64, t3217: f64, t982: f64, t4585: f64, t85: f64, t349: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10190 = t3054 * t1046;
    let t10199 = t9758 * t308;
    let t10202 = t2943 * t1042;
    let t10218 = 0.12841111111111111111e-1_f64 * t9725;
    let t10245 = t982 * t3217;
    let t10269 = t85 * t4585;
    let t10271 = 0.29201909629629629629e-3_f64 * t10269 * t349;
    (t10190, t10199, t10202, t10218, t10245, t10271)
}
