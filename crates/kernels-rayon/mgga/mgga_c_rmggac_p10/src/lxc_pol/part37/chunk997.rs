//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 997/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk997(t78026: f64, t76305: f64, t14451: f64, t1652: f64, t5148: f64, t570: f64, t71910: f64, t8940: f64, t72027: f64, t118: f64, t77416: f64, t76313: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78027 = 0.2993560425465952141e-1_f64 * t78026;
    let t78028 = 0.79828278012425390427e-1_f64 * t76305;
    let t78030 = t5148 * t14451 * t1652;
    let t78031 = 0.2993560425465952141e-1_f64 * t78030;
    let t78034 = 0.11974241701863808564e0_f64 * t8940 * t71910 * t570;
    let t78036 = 0.11974241701863808564e0_f64 * t72027;
    let t78038 = 0.39914139006212695214e-1_f64 * t118 * t77416;
    let t78039 = 0.20455996240684006296e-1_f64 * t76313;
    (t78027, t78028, t78031, t78034, t78036, t78038, t78039)
}
