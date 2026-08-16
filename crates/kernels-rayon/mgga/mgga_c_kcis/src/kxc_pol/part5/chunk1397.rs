//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1397/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1397(t1591: f64, t7490: f64, t18059: f64, t18060: f64, t21933: f64, t21937: f64, t21941: f64, t21945: f64, t21949: f64, t21958: f64, t21961: f64, t21963: f64, t21965: f64) -> (f64, f64) {
    let t23036 = t7490 * t1591;
    let t23052 = -0.19345601851851851852e-2_f64 * t21933 + 0.12897067901234567901e-2_f64 * t21937 - 0.11607361111111111111e-1_f64 * t21941 + 0.51588271604938271605e-2_f64 * t21945 - 0.77382407407407407408e-2_f64 * t21949 - t18059 + t18060 - 0.17411041666666666666e-2_f64 * t21958 + 0.11607361111111111111e-2_f64 * t21961 - 0.25794135802469135802e-3_f64 * t21963 + 0.23214722222222222221e-2_f64 * t21965;
    (t23036, t23052)
}
