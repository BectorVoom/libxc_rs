//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 865/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk865<F: Float>(t262: F, t39063: F, t7204: F, t3807: F, t8639: F, t8642: F, t1462: F, t236: F, t498: F, t7231: F, t8517: F, t34903: F, t34905: F, t34907: F, t34911: F, t34913: F, t39031: F, t39033: F, t39036: F, t39039: F, t39042: F, t39046: F, t39048: F, t39057: F, t39061: F) -> (F, F) {
    let t39064 = t262 * t39063;
    let t39065 = t7204 * t39064;
    let t39068 = t3807 * t8639 * t8642;
    let t39073 = t8517 * t7231 * t236 * t1462 * t498;
    let t39075 = -F::cast_from(0.54549323308490683456e-1_f64) * t39031 - F::cast_from(0.34093327067806677161e-2_f64) * t39033 + F::cast_from(0.81823984962736025184e-1_f64) * t39036 + F::cast_from(0.40911992481368012593e-1_f64) * t39039 + F::cast_from(0.2993560425465952141e-1_f64) * t39042 + F::cast_from(0.20455996240684006296e-1_f64) * t39046 + F::cast_from(0.72732431077987577942e-1_f64) * t39048 + F::cast_from(0.24829349937757072982e-4_f64) * t34903 + F::cast_from(0.12414674968878536491e-4_f64) * t34905 + F::cast_from(0.19863479950205658386e-4_f64) * t34907 + F::cast_from(0.29795219925308487579e-4_f64) * t34911 - F::cast_from(0.29795219925308487579e-4_f64) * t34913 + F::cast_from(0.40911992481368012592e-1_f64) * t39057 - F::cast_from(0.81823984962736025184e-1_f64) * t39061 - F::cast_from(0.20455996240684006296e-1_f64) * t39065 + F::cast_from(0.40911992481368012592e-1_f64) * t39068 - F::cast_from(0.23942587439980034662e-4_f64) * t39073;
    (t39064, t39075)
}
