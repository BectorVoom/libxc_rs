//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 938/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk938(t1390: f64, t301: f64, t1310: f64, t12829: f64, t403: f64, t12830: f64, t1311: f64, t164: f64, t1314: f64, t1309: f64, t3966: f64, t3974: f64) -> (f64, f64, f64, f64, f64) {
    let t13893 = 1.0_f64 / t301 / t1390;
    let t13894 = t1310 * t13893;
    let t13895 = t403 * t12829;
    let t13896 = t13895 * t12830;
    let t13897 = t13894 * t13896;
    let t13900 = t164 * t1311;
    let t13901 = t13900 * t1314;
    let t13902 = t1309 * t13901;
    let t13906 = t3966 * t3974;
    (t13894, t13897, t13900, t13902, t13906)
}
