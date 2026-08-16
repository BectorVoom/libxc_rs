//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 976/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk976(t71112: f64, t68910: f64, t74753: f64, t74772: f64, t74775: f64, t77183: f64, t77184: f64, t77185: f64, t77186: f64, t77187: f64, t77189: f64, t77190: f64, t77191: f64, t77192: f64, t77193: f64, t77195: f64, t77196: f64) -> f64 {
    let t77197 = 0.29795219925308487579e-4_f64 * t71112;
    let t77200 = t77183 - t77184 - t77185 + t77186 + t77187 + 0.17451485956252114154e-4_f64 * t74753 + t77189 - t77190 + t77191 - t77192 - t77193 - 0.13139479569676025391e-5_f64 * t74772 - t77195 - t77196 + t77197 - 0.4379826523225341797e-6_f64 * t74775 - 0.16566831523319392755e-1_f64 * t68910;
    t77200
}
