//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 624/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk624<F: Float>(t1335: F, t6985: F, t1316: F, t3901: F, t6953: F, t3899: F, t3905: F, t5469: F, t6939: F, t6942: F, t6946: F, t482: F) -> (F, F, F, F, F, F) {
    let t6986 = t6985 * t1335;
    let t6988 = F::cast_from(1.0_f64) * t1316 * t6986;
    let t6989 = t6953 * t3901;
    let t6991 = F::cast_from(0.16081824322151104822e2_f64) * t3899 * t6989;
    let t6996 = t3905 + F::cast_from(0.61805555555555555556e-2_f64) * t5469 - F::cast_from(0.61805555555555555555e-2_f64) * t6939 + F::cast_from(0.18541666666666666667e-1_f64) * t6942 - F::cast_from(0.92708333333333333333e-2_f64) * t6946;
    let t6997 = t6996 * t482;
    (t6986, t6988, t6989, t6991, t6996, t6997)
}
