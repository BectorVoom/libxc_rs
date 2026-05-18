//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1033/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1033<F: Float>(t2526: F, t6212: F, t19790: F, t910: F, t146: F, t5094: F, t774: F, t560: F, t7977: F, t481: F, t1234: F, t2841: F) -> (F, F, F, F, F, F) {
    let t24912 = t6212 * t2526;
    let t24916 = t19790 * t910;
    let t25169 = t146 * t5094 * t774;
    let t25172 = t7977 * t560;
    let t25177 = t7977 * t481;
    let t25183 = t2841 * t1234;
    (t24912, t24916, t25169, t25172, t25177, t25183)
}
