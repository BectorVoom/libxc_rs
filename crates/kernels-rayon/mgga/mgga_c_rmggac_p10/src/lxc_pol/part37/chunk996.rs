//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 996/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk996(t76279: f64, t76281: f64, t76283: f64, t76285: f64, t76287: f64, t76289: f64, t2123: f64, t9530: f64, t118: f64, t5259: f64, t551: f64, t71903: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78012 = 0.13637330827122670865e-1_f64 * t76279;
    let t78017 = 0.81823984962736025184e-1_f64 * t76281;
    let t78018 = 0.20455996240684006296e-1_f64 * t76283;
    let t78019 = 0.81823984962736025184e-1_f64 * t76285;
    let t78020 = 0.20455996240684006296e0_f64 * t76287;
    let t78021 = 0.40911992481368012592e-1_f64 * t76289;
    let t78022 = t9530 * t2123;
    let t78024 = 0.39914139006212695214e-1_f64 * t118 * t78022;
    let t78026 = t5259 * t71903 * t551;
    (t78012, t78017, t78018, t78019, t78020, t78021, t78022, t78024, t78026)
}
