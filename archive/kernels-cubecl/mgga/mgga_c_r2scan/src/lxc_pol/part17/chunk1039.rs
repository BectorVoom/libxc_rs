//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1039/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1039<F: Float>(t2719: F, t6212: F, t19790: F, t938: F, t2526: F, t910: F, t146: F, t5094: F, t774: F, t921: F, t2654: F, t2625: F) -> (F, F, F, F, F, F, F, F) {
    let t24902 = t6212 * t2719;
    let t24906 = t19790 * t938;
    let t24912 = t6212 * t2526;
    let t24916 = t19790 * t910;
    let t25169 = t146 * t5094 * t774;
    let t25397 = t19790 * t921;
    let t25480 = t6212 * t2654;
    let t25486 = t6212 * t2625;
    (t24902, t24906, t24912, t24916, t25169, t25397, t25480, t25486)
}
