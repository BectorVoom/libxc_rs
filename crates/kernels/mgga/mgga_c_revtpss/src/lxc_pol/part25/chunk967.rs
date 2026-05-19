//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 967/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk967<F: Float>(t11408: F, t302: F, t2944: F, t953: F, t2970: F, t11132: F, t11337: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11339: F, t11343: F, t11346: F) -> (F, F, F, F) {
    let t11409 = t302 * t11408;
    let t11410 = t2944 * t953;
    let t11411 = t11410 * t2970;
    let t11422 = F::cast_from(0.16068111111111111111e1_f64) * t11132;
    let t11423 = F::cast_from(0.46308888888888888888e0_f64) * t11337;
    let t11428 = F::new(0.6311625e0) * t11316 - F::new(0.104195e0) * t11319 + F::new(0.62517e0) * t11322 + F::new(0.309885e1) * t11167 - F::new(0.103295e1) * t11158 - F::cast_from(0.41678000000000000001e0_f64) * t11326 + F::new(0.20839e0) * t11329 - F::new(0.62517e0) * t11332 - t11422 - t11423 + F::cast_from(0.69463333333333333335e-1_f64) * t11339 - F::cast_from(0.46308888888888888889e-1_f64) * t11343 - F::new(0.104195e0) * t11346 - F::new(0.309885e1) * t11162;
    (t11409, t11410, t11411, t11428)
}
