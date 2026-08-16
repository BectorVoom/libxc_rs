//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1985/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1985(t102468: f64, t108508: f64, t108510: f64, t108512: f64, t108514: f64, t108516: f64, t108518: f64, t108520: f64, t108522: f64, t108524: f64, t108526: f64, t108528: f64) -> f64 {
    let t109777 = -t102468 + 0.34299214494455789578e-2_f64 * t108508 - 0.17149607247227894789e-2_f64 * t108510 + 0.17149607247227894789e-2_f64 * t108512 + 0.51448821741683684367e-2_f64 * t108514 - 0.32012600194825403606e-1_f64 * t108516 - 0.51448821741683684367e-2_f64 * t108518 - 0.17149607247227894789e-1_f64 * t108520 + 0.34299214494455789578e-2_f64 * t108522 + 0.40656002247428262581e-3_f64 * t108524 + 0.34299214494455789578e-2_f64 * t108526 - 0.85748036236139473944e-3_f64 * t108528;
    t109777
}
