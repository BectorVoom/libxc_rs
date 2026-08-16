//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1044/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1044(t72027: f64, t118: f64, t77416: f64, t76313: f64, t76315: f64, t352: f64, t77960: f64, t8940: f64, t25877: f64, t77094: f64, t25854: f64, t77097: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78036 = 0.11974241701863808564e0_f64 * t72027;
    let t78038 = 0.39914139006212695214e-1_f64 * t118 * t77416;
    let t78039 = 0.20455996240684006296e-1_f64 * t76313;
    let t78040 = 0.20455996240684006296e-1_f64 * t76315;
    let t78046 = 0.11974241701863808564e0_f64 * t8940 * t77960 * t352;
    let t78047 = t25877 * t77094;
    let t78048 = 0.17961362552795712846e0_f64 * t78047;
    let t78049 = t25854 * t77097;
    (t78036, t78038, t78039, t78040, t78046, t78048, t78049)
}
