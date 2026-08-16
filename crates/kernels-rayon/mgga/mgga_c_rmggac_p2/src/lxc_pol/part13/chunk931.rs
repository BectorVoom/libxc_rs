//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 931/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk931(t35195: f64, t40450: f64, t1184: f64, t1971: f64, t236: f64, t36489: f64, t40064: f64, t2868: f64, t7779: f64, t2186: f64, t8597: f64, t2412: f64, t7404: f64) -> (f64, f64, f64, f64, f64) {
    let t40451 = t40450 * t35195;
    let t40456 = t36489 * t1971 * t236 * t40064 * t1184;
    let t40458 = t2868 * t7779;
    let t40479 = t2186 * t8597;
    let t40481 = t2412 * t7404;
    (t40451, t40456, t40458, t40479, t40481)
}
