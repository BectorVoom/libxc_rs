//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1067/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1067(t15887: f64, t290: f64, t289: f64, t77283: f64, t77286: f64, t77287: f64, t77288: f64, t77293: f64, t77297: f64, t77299: f64, t77300: f64, t77301: f64, t77303: f64, t77305: f64, t77309: f64, t77313: f64, t77317: f64, t77321: f64, t77322: f64) -> f64 {
    let t80183 = t290 * t15887;
    let t80186 = -t77283 + t77286 - t77287 + t77288 + t77293 - t77297 + t77299 - t77300 + t77301 + t77303 - t77305 - t77309 + t77313 - t77317 + t77321 - 0.2363e1_f64 * t289 * t80183 - t77322;
    t80186
}
