//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 957/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk957<F: Float>(t3216: F, t560: F, t551: F, t552: F, t481: F, t3016: F) -> (F, F, F, F, F, F, F) {
    let t9098 = t3216 * t560;
    let t9100 = t551 * t552 * t9098;
    let t9103 = t3216 * t481;
    let t9105 = t551 * t552 * t9103;
    let t9110 = t3016 * t560;
    let t9112 = t551 * t552 * t9110;
    let t9115 = t3016 * t481;
    (t9098, t9100, t9103, t9105, t9110, t9112, t9115)
}
