//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 939/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk939(t74456: f64, t74459: f64, t74465: f64, t637: f64, t8641: f64, t71772: f64, t8645: f64, t71163: f64, t8649: f64, t71167: f64, t70948: f64, t74487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77014 = 0.5107751987195740728e-4_f64 * t74456;
    let t77015 = 0.1702583995731913576e-4_f64 * t74459;
    let t77017 = 0.15961724959986689775e-4_f64 * t74465;
    let t77018 = t637 * t8641;
    let t77019 = t71772 * t77018;
    let t77020 = 0.20455996240684006296e-1_f64 * t77019;
    let t77021 = t637 * t8645;
    let t77022 = t71163 * t77021;
    let t77023 = 0.40911992481368012592e-1_f64 * t77022;
    let t77024 = t637 * t8649;
    let t77025 = t71167 * t77024;
    let t77026 = 0.20455996240684006296e-1_f64 * t77025;
    let t77031 = 0.90915538847484472429e-2_f64 * t70948;
    let t77034 = 0.40911992481368012592e-1_f64 * t74487;
    (t77014, t77015, t77017, t77020, t77023, t77026, t77031, t77034)
}
