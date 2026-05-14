//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 774/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk774<F: Float>(t34903: F, t34905: F, t34907: F, t34911: F, t34913: F, t39031: F, t39033: F, t39036: F, t39039: F, t39042: F, t39046: F, t39048: F, t39057: F, t39061: F, t39065: F, t39068: F, t39073: F) -> (F,) {
    let t39075 = -0.54549323308490683456e-1 * t39031 - 0.34093327067806677161e-2 * t39033 + 0.81823984962736025184e-1 * t39036 + 0.40911992481368012593e-1 * t39039 + 0.2993560425465952141e-1 * t39042 + 0.20455996240684006296e-1 * t39046 + 0.72732431077987577942e-1 * t39048 + 0.24829349937757072982e-4 * t34903 + 0.12414674968878536491e-4 * t34905 + 0.19863479950205658386e-4 * t34907 + 0.29795219925308487579e-4 * t34911 - 0.29795219925308487579e-4 * t34913 + 0.40911992481368012592e-1 * t39057 - 0.81823984962736025184e-1 * t39061 - 0.20455996240684006296e-1 * t39065 + 0.40911992481368012592e-1 * t39068 - 0.23942587439980034662e-4 * t39073;
    (t39075,)
}
