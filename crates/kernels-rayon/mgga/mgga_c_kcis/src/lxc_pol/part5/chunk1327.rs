//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1327/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1327(t21936: f64, t5661: f64, t17009: f64, t20984: f64, t4170: f64, t16693: f64, t16692: f64, t21655: f64, t5662: f64, t16788: f64, t16793: f64, t21453: f64, t21919: f64, t21923: f64, t21926: f64, t21929: f64, t21933: f64, t507: f64) -> (f64, f64, f64, f64, f64) {
    let t21937 = t5661 * t21936;
    let t21939 = t17009 * t20984;
    let t21940 = t4170 * t21939;
    let t21941 = t5661 * t21940;
    let t21943 = t16693 * t20984;
    let t21944 = t4170 * t21943;
    let t21945 = t16692 * t21944;
    let t21947 = t5662 * t21655;
    let t21948 = t4170 * t21947;
    let t21949 = t16788 * t21948;
    let t21951 = t21453 * t507 + 0.33163888888888888888e-2_f64 * t21919 - 0.22109259259259259259e-2_f64 * t16793 + 0.88437037037037037034e-2_f64 * t21923 - 0.58958024691358024689e-2_f64 * t21926 + 0.17687407407407407407e-1_f64 * t21929 - 0.27636574074074074073e-2_f64 * t21933 + 0.18424382716049382715e-2_f64 * t21937 - 0.16581944444444444444e-1_f64 * t21941 + 0.73697530864197530861e-2_f64 * t21945 - 0.11054629629629629629e-1_f64 * t21949;
    (t21937, t21941, t21945, t21949, t21951)
}
