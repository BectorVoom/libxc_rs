//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 832/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk832<F: Float>(t6263: F, t783: F, t785: F, t1610: F, t1616: F, t20: F, t489: F, t524: F, t525: F) -> (F, F, F, F) {
    let t6266 = 0.73613752582167450608e0 * t783 * t785 * t6263;
    let t6268 = t783 * t1610 * t1616;
    let t6291 = t489 * t20;
    let t6293 = t524 * t525 * t6291;
    (t6266, t6268, t6291, t6293)
}
