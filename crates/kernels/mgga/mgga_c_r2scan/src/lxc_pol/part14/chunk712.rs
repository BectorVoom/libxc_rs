//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 712/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk712<F: Float>(t6263: F, t783: F, t785: F, t1610: F, t1616: F, t1234: F, t133: F, t1605: F, t1604: F, t20: F, t489: F, t524: F, t525: F, t2135: F, t2294: F, t2133: F) -> (F, F, F, F, F, F) {
    let t6266 = 0.73613752582167450608e0 * t783 * t785 * t6263;
    let t6268 = t783 * t1610 * t1616;
    let t6271 = t1605 * t133 * t1234;
    let t6272 = t1604 * t6271;
    let t6291 = t489 * t20;
    let t6293 = t524 * t525 * t6291;
    let t6303 = t2294 * t2135;
    let t6304 = t2133 * t6303;
    (t6266, t6268, t6271, t6272, t6293, t6304)
}
