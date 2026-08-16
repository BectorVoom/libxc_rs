//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1170/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1170(t1797: f64, t3343: f64, t1809: f64, t3348: f64, t13321: f64, t381: f64, t3444: f64, t10513: f64, t284: f64, t14616: f64, t5047: f64, t10753: f64, t5099: f64) -> (f64, f64, f64, f64, f64) {
    let t14825 = t1797 * t3343;
    let t14827 = t1809 * t3348;
    let t14829 = t13321 * t381;
    let t14830 = t14829 * t3444;
    let t14832 = t10513 * t284;
    let t14833 = t5047 * t14616;
    let t14834 = t14832 * t14833;
    let t14836 = t10753 * t5099;
    (t14825, t14827, t14830, t14834, t14836)
}
