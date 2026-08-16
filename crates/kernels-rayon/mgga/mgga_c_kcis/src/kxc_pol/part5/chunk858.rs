//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 858/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk858(t1335: f64, t6985: f64, t1316: f64, t3901: f64, t6953: f64, t3899: f64, t3905: f64, t5469: f64, t6939: f64, t6942: f64, t6946: f64, t482: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6986 = t6985 * t1335;
    let t6988 = 1.0_f64 * t1316 * t6986;
    let t6989 = t6953 * t3901;
    let t6991 = 0.16081824322151104822e2_f64 * t3899 * t6989;
    let t6996 = t3905 + 0.61805555555555555556e-2_f64 * t5469 - 0.61805555555555555555e-2_f64 * t6939 + 0.18541666666666666667e-1_f64 * t6942 - 0.92708333333333333333e-2_f64 * t6946;
    let t6997 = t6996 * t482;
    (t6986, t6988, t6989, t6991, t6996, t6997)
}
