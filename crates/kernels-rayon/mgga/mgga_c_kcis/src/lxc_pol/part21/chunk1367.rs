//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1367/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1367(t28219: f64, t7784: f64, t7772: f64, t97024: f64, t1856: f64, t26996: f64, t3611: f64, t5329: f64, t30066: f64, t3532: f64, t27042: f64, t28113: f64, t28118: f64, t28125: f64, t28153: f64, t92795: f64, t93023: f64, t93028: f64, t93082: f64) -> (f64, f64, f64) {
    let t97248 = 0.23168402777777777778e-3_f64 * t28219 * t7784;
    let t97250 = 0.30918233506944444444e-4_f64 * t7772 * t97024;
    let t97253 = t5329 * t26996 * t1856 * t3611;
    let t97258 = t5329 * t30066 * t1856 * t3532;
    let t97263 = 0.46336805555555555556e-3_f64 * t93023 * t28118 + 0.30918233506944444444e-4_f64 * t93028 * t28113 - 0.30891203703703703704e-3_f64 * t93023 * t28125 - 0.12356481481481481482e-2_f64 * t92795 * t28118 - 0.82448622685185185185e-4_f64 * t93082 * t28113 + 0.8237654320987654321e-3_f64 * t92795 * t28125 - t97248 - t97250 - 0.46377350260416666667e-4_f64 * t7772 * t97253 + 0.92754700520833333334e-4_f64 * t7772 * t97258 - 0.24734586805555555556e-3_f64 * t27042 * t28153;
    (t97253, t97258, t97263)
}
