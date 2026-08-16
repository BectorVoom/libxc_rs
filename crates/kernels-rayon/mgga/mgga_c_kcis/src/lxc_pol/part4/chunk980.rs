//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 980/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk980(t1042: f64, t2943: f64, t3093: f64, t932: f64, t9725: f64, t2861: f64, t3184: f64, t3217: f64, t982: f64, t1130: f64, t2865: f64, t1014: f64, t3241: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10202 = t2943 * t1042;
    let t10208 = t932 * t3093;
    let t10218 = 0.12841111111111111111e-1_f64 * t9725;
    let t10243 = t2861 * t3184;
    let t10245 = t982 * t3217;
    let t10250 = t2865 * t1130;
    let t10255 = t1014 * t3241;
    (t10202, t10208, t10218, t10243, t10245, t10250, t10255)
}
