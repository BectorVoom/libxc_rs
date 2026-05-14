//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 865/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk865<F: Float>(t11396: F, t981: F, t2935: F, t945: F, t2967: F, t941: F, t2966: F, t307: F, t302: F, t2944: F, t953: F, t2970: F, t11132: F, t11337: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11339: F, t11343: F, t11346: F) -> (F, F, F, F, F, F, F) {
    let t11398 = 0.51947577317044391277e2 * t981 * t11396;
    let t11399 = t2935 * t945;
    let t11404 = t941 * t2967;
    let t11408 = 1.0 / t2966 / t307;
    let t11409 = t302 * t11408;
    let t11410 = t2944 * t953;
    let t11411 = t11410 * t2970;
    let t11422 = 0.16068111111111111111e1 * t11132;
    let t11423 = 0.46308888888888888888e0 * t11337;
    let t11428 = 0.6311625e0 * t11316 - 0.104195e0 * t11319 + 0.62517e0 * t11322 + 0.309885e1 * t11167 - 0.103295e1 * t11158 - 0.41678000000000000001e0 * t11326 + 0.20839e0 * t11329 - 0.62517e0 * t11332 - t11422 - t11423 + 0.69463333333333333335e-1 * t11339 - 0.46308888888888888889e-1 * t11343 - 0.104195e0 * t11346 - 0.309885e1 * t11162;
    (t11398, t11399, t11404, t11409, t11410, t11411, t11428)
}
