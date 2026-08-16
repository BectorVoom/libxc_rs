//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 987/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk987<F: Float>(t1224: F, t1225: F, t30298: F, t30273: F, t30238: F, t13686: F, t20292: F, t26138: F, t26150: F, t26159: F, t30288: F, t30292: F, t30296: F) -> (F, F, F, F) {
    let t30300 = t1224 * t1225 * t30298;
    let t30303 = t1224 * t1225 * t30273;
    let t30306 = t1224 * t1225 * t30238;
    let t30308 = -t13686 - F::cast_from(0.12361111111111111111e-1_f64) * t20292 + F::cast_from(0.61805555555555555556e-2_f64) * t26138 - F::cast_from(0.18541666666666666667e-1_f64) * t26150 + F::cast_from(0.92708333333333333334e-2_f64) * t26159 - F::cast_from(0.10300925925925925926e-1_f64) * t30288 + F::cast_from(0.37083333333333333333e-1_f64) * t30292 - F::cast_from(0.18541666666666666666e-1_f64) * t30296 - F::cast_from(0.55625000000000000001e-1_f64) * t30300 + F::cast_from(0.55625000000000000001e-1_f64) * t30303 - F::cast_from(0.92708333333333333333e-2_f64) * t30306;
    (t30300, t30303, t30306, t30308)
}
