//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 961/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk961(t23976: f64, t23978: f64, t24608: f64, t2648: f64, t28797: f64, t28803: f64, t28807: f64, t28811: f64, t28815: f64, t28818: f64, t28953: f64, t29759: f64, t29981: f64, t5445: f64) -> f64 {
    let t30020 = 0.18571777777777777778e-1_f64 * t28797 + 0.18571777777777777778e-1_f64 * t23976 - 0.11607361111111111111e-2_f64 * t28803 - 0.92858888888888888888e-2_f64 * t28807 - 0.15476481481481481482e-1_f64 * t28811 - 0.11607361111111111111e-1_f64 * t28815 - 0.69644166666666666666e-2_f64 * t28818 + 0.46429444444444444443e-2_f64 * t23978 - 0.579e0_f64 * t24608 * t2648 - 0.223494e0_f64 * t5445 * t29759 - 0.17411041666666666666e-2_f64 * t28953 + 0.223494e0_f64 * t5445 * t29981;
    t30020
}
