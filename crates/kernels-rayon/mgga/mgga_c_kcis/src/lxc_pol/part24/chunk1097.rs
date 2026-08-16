//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1097/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1097(t14628: f64, t1773: f64, t26760: f64, t1092: f64, t27895: f64, t27947: f64, t28948: f64, t28952: f64, t28967: f64, t28974: f64, t28984: f64, t28988: f64, t7690: f64, t7703: f64, t8030: f64, t8034: f64, t8042: f64) -> (f64, f64, f64, f64) {
    let t28991 = t14628 * t1773;
    let t28992 = t26760 * t28991;
    let t28993 = t1092 * t28992;
    let t28995 = 0.33163888888888888888e-2_f64 * t28967 - 0.2782641015625e-3_f64 * t7690 * t28952 + 0.13901041666666666667e-2_f64 * t8030 * t8042 - 0.24872916666666666666e-2_f64 * t28974 + 0.13901041666666666667e-2_f64 * t8030 * t8034 + 0.18550940104166666667e-3_f64 * t27895 * t8034 + 0.92754700520833333333e-4_f64 * t7690 * t28948 + 0.33163888888888888888e-2_f64 * t27947 + 0.46336805555555555556e-3_f64 * t7703 * t28984 - 0.13901041666666666667e-2_f64 * t7703 * t28988 - 0.33163888888888888888e-2_f64 * t28993;
    (t28991, t28992, t28993, t28995)
}
