//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 886/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk886(t11182: f64, t11184: f64, t11187: f64, t11194: f64, t11272: f64, t11280: f64, t1129: f64, t11297: f64, t11300: f64, t11303: f64, t11307: f64, t11310: f64, t11311: f64, t11345: f64, t11350: f64, t11353: f64, t11356: f64, t11361: f64, t1157: f64, t3334: f64, t3357: f64, t3371: f64, t3378: f64, t3396: f64, t3401: f64, t3404: f64) -> f64 {
    let t11364 = -t11182 - t11184 - t11187 + t11194 - t11272 - t11280 - 0.35089341735807877242e1_f64 * t11297 * t3378 + 0.35089341735807877242e1_f64 * t3401 * t11300 - 6.0_f64 * t11303 * t3334 + 6.0_f64 * t3357 * t11307 + 0.10254018858216406658e4_f64 * t11310 * t11311 + 1.0_f64 * t1129 * t11345 + 0.2069040516770936012e4_f64 * t11350 * t11353 + 0.17544670867903938621e1_f64 * t11356 * t1157 + 0.17544670867903938621e1_f64 * t3371 * t3396 + 0.51947577317044391276e2_f64 * t11361 * t3404;
    t11364
}
