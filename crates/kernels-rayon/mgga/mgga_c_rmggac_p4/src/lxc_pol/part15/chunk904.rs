//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 904/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk904(t118: f64, t128: f64, t2001: f64, t6261: f64, t675: f64, t10010: f64, t2604: f64, t2868: f64, t39048: f64, t45120: f64, t45123: f64, t45126: f64, t45129: f64, t45132: f64, t45135: f64, t45139: f64, t45149: f64, t45152: f64, t45155: f64, t45158: f64, t6434: f64, t6449: f64, t665: f64, t8994: f64, t903: f64) -> f64 {
    let t45163 = t675 * t2001 * t118 * t128 * t6261;
    let t45165 = 0.81823984962736025184e-1_f64 * t45120 - 0.13637330827122670864e0_f64 * t45123 - 0.54549323308490683456e-1_f64 * t45126 - 0.40911992481368012592e-1_f64 * t45129 + 0.81823984962736025184e-1_f64 * t45132 + 0.40911992481368012592e-1_f64 * t45135 - 0.11974241701863808564e0_f64 * t2868 * t8994 + 0.20455996240684006296e-1_f64 * t45139 + 0.35922725105591425692e0_f64 * t903 * t665 * t6449 + 0.35922725105591425692e0_f64 * t903 * t665 * t6434 + 0.23948483403727617128e0_f64 * t2604 * t10010 - 0.90915538847484472429e-2_f64 * t45149 + 0.72042316457491791906e-3_f64 * t45152 - 0.10248087766267884742e-3_f64 * t45155 + 0.72732431077987577943e-1_f64 * t39048 - 0.27274661654245341728e-1_f64 * t45158 - 0.42564599893297839398e-5_f64 * t45163;
    t45165
}
