//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 960/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk960(t1184: f64, t1971: f64, t40444: f64, t511: f64, t7365: f64, t35190: f64, t8450: f64, t35195: f64, t236: f64, t36489: f64, t40064: f64, t2868: f64, t7779: f64) -> (f64, f64, f64, f64) {
    let t40448 = t7365 * t1971 * t511 * t40444 * t1184;
    let t40450 = t8450 * t35190;
    let t40451 = t40450 * t35195;
    let t40456 = t36489 * t1971 * t236 * t40064 * t1184;
    let t40458 = t2868 * t7779;
    (t40448, t40451, t40456, t40458)
}
