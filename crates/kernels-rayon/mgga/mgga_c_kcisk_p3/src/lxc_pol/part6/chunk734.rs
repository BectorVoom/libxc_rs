//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 734/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk734(t10471: f64, t1337: f64, t140: f64, t3480: f64, t3737: f64, t213: f64, t300: f64, t425: f64, t11525: f64, t435: f64, t437: f64, t11529: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13955 = t140 * t10471 * t1337;
    let t13959 = t140 * t3737 * t3480;
    let t13964 = t213 * t300;
    let t13966 = 0.14055920378328537299e-1_f64 * t13964 * t425;
    let t14056 = 0.77488888888888888888e-2_f64 * t435 * t11525 * t437;
    let t14057 = t11529 * t447;
    (t13955, t13959, t13964, t13966, t14056, t14057)
}
