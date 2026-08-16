//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 824/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk824(t3217: f64, t982: f64, t1130: f64, t2865: f64, t1014: f64, t3241: f64, t3238: f64, t4585: f64, t85: f64, t349: f64, t1098: f64, t3290: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10245 = t982 * t3217;
    let t10250 = t2865 * t1130;
    let t10255 = t1014 * t3241;
    let t10257 = t1014 * t3238;
    let t10269 = t85 * t4585;
    let t10271 = 0.29201909629629629629e-3_f64 * t10269 * t349;
    let t10282 = t1098 * t3290;
    (t10245, t10250, t10255, t10257, t10269, t10271, t10282)
}
