//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1312/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1312(t7968: f64, t99175: f64, t3805: f64, t6159: f64, t6207: f64, t3797: f64, t6151: f64, t27567: f64, t27583: f64, t28805: f64, t4440: f64, t94664: f64, t94669: f64, t98942: f64, t98946: f64, t99569: f64) -> f64 {
    let t99678 = t7968 * t99175;
    let t99686 = t6159 * t6207 * t3805;
    let t99690 = t6151 * t6207 * t3797;
    let t99702 = -0.10306077835648148148e-4_f64 * t99678 + 0.11607361111111111111e-2_f64 * t94664 - 0.77382407407407407406e-3_f64 * t94669 - 0.23168402777777777778e-3_f64 * t27583 * t99569 + 0.10317654320987654321e-2_f64 * t98942 + 0.11584201388888888889e-3_f64 * t27583 * t99686 + 0.15445601851851851852e-3_f64 * t27583 * t99690 + 0.15459116753472222222e-4_f64 * t27567 * t99686 + 0.20612155671296296296e-4_f64 * t27567 * t99690 + 0.11584201388888888889e-3_f64 * t27583 * t4440 * t28805 * t3805 - 0.41270617283950617282e-2_f64 * t98946;
    t99702
}
