//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1115/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1115<F: Float>(t5213: F, t9718: F, t2799: F, t5219: F, t11701: F, t1957: F, t5218: F, t5339: F, t5273: F, t736: F, t5277: F, t654: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33079 = 2.0 * t5213 * t9718;
    let t33080 = t2799 * t5219;
    let t33082 = 6.0 * t11701 * t33080;
    let t33083 = t9718 * t1957;
    let t33085 = 4.0 * t5218 * t33083;
    let t33086 = t2799 * t5339;
    let t33088 = 2.0 * t5218 * t33086;
    let t33089 = t5273 * t736;
    let t33091 = t5277 * t654;
    (t33079, t33080, t33082, t33083, t33085, t33086, t33088, t33089, t33091)
}
