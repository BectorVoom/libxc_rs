//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1097/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1097<F: Float>(t19518: F, t19589: F, t19650: F, t19716: F, t41: F, t61: F, t1380: F, t1409: F, t1499: F, t234: F, t1422: F, t1510: F, t5002: F, t732: F, t19576: F, t446: F, t453: F) -> (F, F, F, F, F) {
    let t19720 = t41 * t61 * (t19518 + t19589 + t19650 + t19716);
    let t19728 = 0.21053605041484726346e2 * t234 * t1380 * t1409 * t1499;
    let t19737 = t1422 * t1510;
    let t19743 = t732 * t5002;
    let t19748 = 0.5848223622634646207e0 * t234 * t446 * t19576 * t453;
    (t19720, t19728, t19737, t19743, t19748)
}
