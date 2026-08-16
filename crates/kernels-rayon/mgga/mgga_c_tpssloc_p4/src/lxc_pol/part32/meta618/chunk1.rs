//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2022/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2022(t7291: f64, t85660: f64, t11605: f64, t225: f64, t7303: f64, t1235: f64, t24594: f64, t1176: f64, t1184: f64, t24847: f64, t974: f64, t1009: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85661 = t85660 * t7291;
    let t85674 = t225 * t11605;
    let t85701 = t85660 * t7303;
    let t85807 = t24594 * t1235;
    let t85820 = t24847 * t974 * t1176 * t1184;
    let t85821 = t460 * t1009;
    (t85661, t85674, t85701, t85807, t85820, t85821)
}
