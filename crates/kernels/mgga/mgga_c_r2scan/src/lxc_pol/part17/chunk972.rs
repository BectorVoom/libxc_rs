//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 972/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk972<F: Float>(t1120: F, t6692: F, t37038: F, t37075: F, t1299: F, t3506: F, t3552: F, t6755: F, t1142: F, t19309: F, t6767: F, t19327: F, t1114: F, t23040: F, t6897: F, t3492: F, t5086: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t38783 = t1120 * t6692;
    let t38792 = 308.0 / 27.0 * t37038;
    let t38808 = 308.0 / 27.0 * t37075;
    let t38839 = t3506 * t1299;
    let t38958 = t6755 * t3552;
    let t38961 = t19309 * t1142;
    let t38971 = t6767 * t3552;
    let t38976 = t19327 * t1142;
    let t39010 = t23040 * t1114;
    let t39030 = t1114 * t6897;
    let t39040 = t5086 * t3492;
    (t38783, t38792, t38808, t38839, t38958, t38961, t38971, t38976, t39010, t39030, t39040)
}
