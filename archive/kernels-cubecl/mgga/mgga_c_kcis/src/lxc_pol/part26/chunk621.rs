//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 621/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk621<F: Float>(t3848: F, t5469: F, t6939: F, t6942: F, t6946: F, t469: F, t1907: F, t5541: F, t1906: F, t1335: F, t3861: F, t1897: F) -> (F, F, F, F, F, F, F) {
    let t6948 = t3848 + F::cast_from(0.11872222222222222222e-1_f64) * t5469 - F::cast_from(0.11872222222222222222e-1_f64) * t6939 + F::cast_from(0.35616666666666666666e-1_f64) * t6942 - F::cast_from(0.17808333333333333333e-1_f64) * t6946;
    let t6950 = F::cast_from(0.62182e-1_f64) * t6948 * t469;
    let t6952 = F::cast_from(2.0_f64) * t5541 * t1907;
    let t6953 = t1906 * t1906;
    let t6954 = t6953 * t1335;
    let t6956 = F::cast_from(2.0_f64) * t3861 * t6954;
    let t6957 = t1897 * t1897;
    (t6948, t6950, t6952, t6953, t6954, t6956, t6957)
}
