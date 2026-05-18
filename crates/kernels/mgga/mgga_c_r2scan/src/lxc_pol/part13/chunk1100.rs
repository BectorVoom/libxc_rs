//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1100/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1100<F: Float>(t481: F, t792: F, t983: F, t1065: F, t39190: F, t10609: F, t1561: F, t97: F, t2625: F, t13908: F, t986: F, t3270: F) -> (F, F, F) {
    let t39192 = t983 * t481 * t792;
    let t39195 = F::new(135.0) / F::new(32.0) * t39190 * t1065 * t39192;
    let t39197 = t97 * t10609 * t1561;
    let t39198 = t2625 * t792;
    let t39201 = F::new(15.0) / F::new(4.0) * t39197 * t1065 * t39198;
    let t39202 = t13908 * t986;
    let t39203 = t3270 * t39202;
    (t39195, t39201, t39203)
}
