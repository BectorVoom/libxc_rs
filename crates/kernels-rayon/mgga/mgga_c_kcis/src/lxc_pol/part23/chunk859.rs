//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 859/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk859(t1335: f64, t16223: f64, t1316: f64, t16048: f64, t16050: f64, t11409: f64, t11411: f64, t11413: f64, t11415: f64, t11520: f64, t16046: f64, t16052: f64, t16057: f64, t16062: f64, t16067: f64, t16071: f64, t16075: f64, t16080: f64, t16084: f64, t16088: f64) -> (f64, f64) {
    let t16224 = t16223 * t1335;
    let t16226 = 1.0_f64 * t1316 * t16224;
    let t16232 = 0.41203703703703703704e-2_f64 * t16048;
    let t16233 = 0.12361111111111111111e-1_f64 * t16050;
    let t16243 = -t11520 - 0.82407407407407407407e-2_f64 * t11409 + 0.20601851851851851852e-2_f64 * t11411 - 0.61805555555555555556e-2_f64 * t11413 + 0.30902777777777777778e-2_f64 * t11415 - 0.41203703703703703704e-2_f64 * t16046 + t16232 - t16233 - 0.67986111111111111113e-1_f64 * t16052 - 0.10300925925925925926e-1_f64 * t16057 + 0.37083333333333333333e-1_f64 * t16062 + 0.24722222222222222222e-1_f64 * t16067 - 0.61805555555555555555e-2_f64 * t16071 - 0.55625000000000000001e-1_f64 * t16075 - 0.74166666666666666668e-1_f64 * t16080 + 0.18541666666666666667e-1_f64 * t16084 + 0.18541666666666666667e-1_f64 * t16088;
    (t16226, t16243)
}
