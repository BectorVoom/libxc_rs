//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1339/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1339(t22142: f64, t5498: f64, t1102: f64, t11632: f64, t11640: f64, t16545: f64, t16547: f64, t16562: f64, t16567: f64, t16587: f64, t1924: f64, t22091: f64, t22095: f64, t22099: f64, t22103: f64, t22107: f64, t22111: f64, t22116: f64, t22120: f64, t22128: f64, t22131: f64, t22135: f64, t22139: f64, t344: f64, t4587: f64, t5623: f64) -> f64 {
    let t22143 = t5498 * t22142;
    let t22146 = -t16545 - t16547 + 0.73004774074074074073e-3_f64 * t22091 - 0.1478346675e-2_f64 * t1102 * t22095 + 0.19711289e-2_f64 * t1102 * t22099 - 0.13140859333333333333e-2_f64 * t1102 * t22103 + 0.26281718666666666666e-2_f64 * t4587 * t22107 - 0.19711289e-2_f64 * t1102 * t22111 + 0.59133867e-2_f64 * t1102 * t22116 - 0.19711289e-2_f64 * t11632 * t22120 - 0.14600954814814814815e-3_f64 * t11640 + t16562 + t16567 - 0.87605728888888888887e-3_f64 * t16587 - 8.0_f64 * t1924 * t5623 + 0.1478346675e-2_f64 * t344 * t22128 - 0.19711289e-2_f64 * t22131 + 0.295669335e-2_f64 * t1102 * t22135 - 0.59133867e-2_f64 * t1102 * t22139 + 0.39422578e-2_f64 * t1102 * t22143;
    t22146
}
