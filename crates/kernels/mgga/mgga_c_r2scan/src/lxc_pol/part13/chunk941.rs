//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 941/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk941<F: Float>(t19790: F, t921: F, t1553: F, t7338: F, t2654: F, t6212: F, t2625: F, t2634: F, t2612: F, t1543: F, t2531: F, t481: F, t113: F, t7197: F, t1550: F, t910: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25397 = t19790 * t921;
    let t25466 = t7338 * t1553;
    let t25480 = t6212 * t2654;
    let t25486 = t6212 * t2625;
    let t25499 = t6212 * t2634;
    let t25503 = t6212 * t2612;
    let t25562 = t921 * t1543;
    let t25569 = t2531 * t481;
    let t25573 = t7197 * t113;
    let t25577 = t2634 * t481;
    let t25670 = t910 * t1550 * t113;
    (t25397, t25466, t25480, t25486, t25499, t25503, t25562, t25569, t25573, t25577, t25670)
}
