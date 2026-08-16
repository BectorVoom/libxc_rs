//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1220/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1220(t118: f64, t2576: f64, t794: f64, t9516: f64, t207: f64, t40394: f64, t40399: f64, t210: f64, t214: f64, t2571: f64, t40848: f64, t40972: f64, t40977: f64, t41142: f64, t41144: f64, t41149: f64, t41151: f64, t41155: f64, t41156: f64, t41158: f64, t41161: f64, t41173: f64, t787: f64) -> f64 {
    let t41181 = t2576 * t118 * t794 * t9516;
    let t41185 = 0.69444444444444444445e-4_f64 * t40394 * t207 * t40399;
    let t41186 = 0.99999999999999999996e-2_f64 * t41142 - 0.79999999999999999997e-1_f64 * t41144 - 0.29999999999999999998e-1_f64 * t41149 + 0.15555555555555555555e-1_f64 * t41151 + t41155 + 0.22469135802469135801e0_f64 * t41156 + 0.18666666666666666665e0_f64 * t41158 + 0.99999999999999999995e-1_f64 * t41161 * t210 * t214 * t40972 + 0.14999999999999999999e-1_f64 * t2571 * t210 * t214 * t40977 + 0.39999999999999999998e-1_f64 * t41173 - 0.16666666666666666666e-2_f64 * t787 * t210 * t214 * t40848 + 0.33333333333333333332e-2_f64 * t41181 - t41185;
    t41186
}
