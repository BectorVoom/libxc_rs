//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1088/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1088(t1347: f64, t2479: f64, t1652: f64, t8264: f64, t2471: f64, t794: f64, t2211: f64, t5249: f64, t1356: f64, t1550: f64, t2604: f64, t27124: f64, t36448: f64, t40683: f64, t40685: f64, t40688: f64, t40690: f64, t40695: f64, t40700: f64, t40703: f64, t40706: f64, t4965: f64, t5928: f64, t739: f64, t8265: f64, t9437: f64, t9531: f64) -> (f64, f64, f64, f64) {
    let t43680 = t1347 * t2479;
    let t43685 = t8264 * t1652;
    let t43692 = t2471 * t794;
    let t43698 = t2211 * t5249;
    let t43708 = t43680 - 0.5107751987195740728e-4_f64 * t40683 + 0.11974241701863808564e0_f64 * t739 * t2211 * t27124 + 0.79828278012425390428e-1_f64 * t1356 * t43685 - 0.11974241701863808564e0_f64 * t40685 - 0.5987120850931904282e-1_f64 * t40688 + 0.79828278012425390428e-1_f64 * t4965 * t9531 - 0.11974241701863808564e0_f64 * t1550 * t43692 - 0.11974241701863808564e0_f64 * t2604 * t9437 + 0.5987120850931904282e-1_f64 * t40690 + 0.39914139006212695214e-1_f64 * t1356 * t43698 + 0.212822999466489197e-4_f64 * t40695 + 0.1702583995731913576e-4_f64 * t40700 - 0.5107751987195740728e-4_f64 * t40703 - 0.5107751987195740728e-4_f64 * t40706 + 0.79828278012425390428e-1_f64 * t5928 * t8265 - 0.11918087970123395032e-3_f64 * t36448;
    (t43685, t43692, t43698, t43708)
}
