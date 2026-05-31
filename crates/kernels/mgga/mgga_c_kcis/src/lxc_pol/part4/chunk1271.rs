//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1271/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1271<F: Float>(t1335: F, t16223: F, t1316: F, t16048: F, t16050: F, t11409: F, t11411: F, t11413: F, t11415: F, t11520: F, t16046: F, t16052: F, t16057: F, t16062: F, t16067: F, t16071: F, t16075: F, t16080: F, t16084: F, t16088: F) -> (F, F) {
    let t16224 = t16223 * t1335;
    let t16226 = F::cast_from(1.0_f64) * t1316 * t16224;
    let t16232 = F::cast_from(0.41203703703703703704e-2_f64) * t16048;
    let t16233 = F::cast_from(0.12361111111111111111e-1_f64) * t16050;
    let t16243 = -t11520 - F::cast_from(0.82407407407407407407e-2_f64) * t11409 + F::cast_from(0.20601851851851851852e-2_f64) * t11411 - F::cast_from(0.61805555555555555556e-2_f64) * t11413 + F::cast_from(0.30902777777777777778e-2_f64) * t11415 - F::cast_from(0.41203703703703703704e-2_f64) * t16046 + t16232 - t16233 - F::cast_from(0.67986111111111111113e-1_f64) * t16052 - F::cast_from(0.10300925925925925926e-1_f64) * t16057 + F::cast_from(0.37083333333333333333e-1_f64) * t16062 + F::cast_from(0.24722222222222222222e-1_f64) * t16067 - F::cast_from(0.61805555555555555555e-2_f64) * t16071 - F::cast_from(0.55625000000000000001e-1_f64) * t16075 - F::cast_from(0.74166666666666666668e-1_f64) * t16080 + F::cast_from(0.18541666666666666667e-1_f64) * t16084 + F::cast_from(0.18541666666666666667e-1_f64) * t16088;
    (t16226, t16243)
}
