//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 865/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk865(t262: f64, t39063: f64, t7204: f64, t3807: f64, t8639: f64, t8642: f64, t1462: f64, t236: f64, t498: f64, t7231: f64, t8517: f64, t34903: f64, t34905: f64, t34907: f64, t34911: f64, t34913: f64, t39031: f64, t39033: f64, t39036: f64, t39039: f64, t39042: f64, t39046: f64, t39048: f64, t39057: f64, t39061: f64) -> (f64, f64) {
    let t39064 = t262 * t39063;
    let t39065 = t7204 * t39064;
    let t39068 = t3807 * t8639 * t8642;
    let t39073 = t8517 * t7231 * t236 * t1462 * t498;
    let t39075 = -0.54549323308490683456e-1_f64 * t39031 - 0.34093327067806677161e-2_f64 * t39033 + 0.81823984962736025184e-1_f64 * t39036 + 0.40911992481368012593e-1_f64 * t39039 + 0.2993560425465952141e-1_f64 * t39042 + 0.20455996240684006296e-1_f64 * t39046 + 0.72732431077987577942e-1_f64 * t39048 + 0.24829349937757072982e-4_f64 * t34903 + 0.12414674968878536491e-4_f64 * t34905 + 0.19863479950205658386e-4_f64 * t34907 + 0.29795219925308487579e-4_f64 * t34911 - 0.29795219925308487579e-4_f64 * t34913 + 0.40911992481368012592e-1_f64 * t39057 - 0.81823984962736025184e-1_f64 * t39061 - 0.20455996240684006296e-1_f64 * t39065 + 0.40911992481368012592e-1_f64 * t39068 - 0.23942587439980034662e-4_f64 * t39073;
    (t39064, t39075)
}
