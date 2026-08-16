//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1332/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1332(t22035: f64, t542: f64, t1098: f64, t7250: f64, t1102: f64, t15994: f64, t16001: f64, t16003: f64, t16038: f64, t21983: f64, t21987: f64, t21990: f64, t21993: f64, t21996: f64, t22001: f64, t22006: f64, t22011: f64, t22015: f64, t22018: f64, t22021: f64, t22025: f64, t22029: f64, t22032: f64, t344: f64, t4587: f64) -> f64 {
    let t22036 = t542 * t22035;
    let t22039 = t1098 * t7250;
    let t22043 = 0.16426074166666666666e-2_f64 * t1102 * t21983 - 0.10950716111111111111e-2_f64 * t1102 * t21987 - 0.65704296666666666666e-2_f64 * t1102 * t21990 + 0.29201909629629629629e-2_f64 * t1102 * t21993 - 0.43802864444444444444e-2_f64 * t4587 * t21996 - 0.65704296666666666667e-3_f64 * t1102 * t22001 + 0.98556445e-3_f64 * t1102 * t22006 + 0.13140859333333333333e-2_f64 * t1102 * t22011 + 0.13140859333333333333e-2_f64 * t1102 * t22015 + 0.39422577999999999999e-2_f64 * t1102 * t22018 + 0.52563437333333333332e-2_f64 * t4587 * t22021 + 0.98556445e-3_f64 * t1102 * t22025 - 0.65704296666666666667e-3_f64 * t1102 * t22029 - 0.13140859333333333333e-2_f64 * t1102 * t22032 - 0.98556445e-3_f64 * t344 * t22036 - 0.65704296666666666667e-3_f64 * t22039 - 0.17521145777777777778e-2_f64 * t15994 + t16001 - t16003 - 0.2920190962962962963e-3_f64 * t16038;
    t22043
}
