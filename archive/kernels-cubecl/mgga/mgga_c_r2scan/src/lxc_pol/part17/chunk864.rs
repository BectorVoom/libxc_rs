//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 864/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk864<F: Float>(t8001: F, t921: F, t360: F, t2531: F, t2562: F, t1632: F, t3071: F, t551: F, t1577: F, t1600: F, t3073: F, t2892: F, t560: F) -> (F, F, F, F, F, F, F) {
    let t9165 = t8001 * t921;
    let t9166 = t360 * t9165;
    let t9169 = t2562 * t2531;
    let t9170 = t360 * t9169;
    let t9177 = t551 * t1632 * t3071;
    let t9178 = t1577 * t9177;
    let t9180 = t1600 * t3073;
    let t9182 = t2892 * t560;
    (t9165, t9166, t9169, t9170, t9178, t9180, t9182)
}
