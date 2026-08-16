//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1343/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1343<F: Float>(t10556: F, t10608: F, t13598: F, t14352: F, t14353: F, t14354: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t17180: F, t17185: F, t17189: F) -> F {
    let t17191 = -t10608 - F::cast_from(0.41203703703703703703e-2_f64) * t10556 - F::cast_from(0.82407407407407407408e-2_f64) * t13598 + t14352 - t14353 + t14354 + F::cast_from(0.20601851851851851852e-2_f64) * t17149 - F::cast_from(0.10300925925925925926e-1_f64) * t17154 + F::cast_from(0.37083333333333333333e-1_f64) * t17159 - F::cast_from(0.12361111111111111111e-1_f64) * t17163 - F::cast_from(0.61805555555555555557e-2_f64) * t17165 - F::cast_from(0.55625000000000000001e-1_f64) * t17169 + F::cast_from(0.37083333333333333334e-1_f64) * t17173 + F::cast_from(0.30902777777777777778e-2_f64) * t17175 - F::cast_from(0.61805555555555555555e-2_f64) * t17180 + F::cast_from(0.18541666666666666667e-1_f64) * t17185 - F::cast_from(0.92708333333333333333e-2_f64) * t17189;
    t17191
}
