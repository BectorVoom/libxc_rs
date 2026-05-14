//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1116/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1116<F: Float>(t20481: F, t481: F, t551: F, t566: F, t133: F, t5052: F, t146: F, t1603: F, t6257: F, t2158: F, t2120: F, t524: F, t6238: F, t546: F, t8028: F, t560: F, t6363: F) -> (F, F, F, F, F, F, F, F) {
    let t20932 = t566 * t551 * t20481 * t481;
    let t20946 = t5052 * t133;
    let t20947 = t146 * t20946;
    let t20954 = t6257 * t1603;
    let t20955 = t20954 * t2158;
    let t20994 = t524 * t6238 * t2120;
    let t20997 = t546 * t8028;
    let t20998 = t6363 * t560;
    (t20932, t20946, t20947, t20954, t20955, t20994, t20997, t20998)
}
