//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 772/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk772(t35959: f64, t3814: f64, t36: f64, t4616: f64, t2064: f64, t839: f64, t5245: f64, t848: f64, t797: f64, t34805: f64, t648: f64, t35765: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35960 = t3814 * t35959;
    let t35972 = t4616 * t36;
    let t35979 = t2064 * t839;
    let t35980 = t3814 * t35979;
    let t35989 = t5245 * t2064;
    let t36012 = t2064 * t848;
    let t36013 = t797 * t36012;
    let t36034 = t648 * t34805;
    let t36035 = 0.15556658869458454171e0_f64 * t36034;
    let t36045 = t793 * t35765;
    (t35960, t35972, t35979, t35980, t35989, t36012, t36013, t36035, t36045)
}
