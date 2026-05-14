//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 990/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk990<F: Float>(t2330: F, t39182: F, t3262: F, t3263: F, t11622: F, t37271: F, t3261: F, t5086: F, t97: F, t481: F, t792: F, t983: F, t1065: F, t10609: F, t1561: F, t2625: F) -> (F, F, F, F, F) {
    let t39183 = t39182 * t2330;
    let t39186 = 3.0 / 2.0 * t3262 * t3263 * t39183;
    let t39188 = 45.0 / 32.0 * t37271 * t11622;
    let t39190 = t97 * t3261 * t5086;
    let t39192 = t983 * t481 * t792;
    let t39195 = 135.0 / 32.0 * t39190 * t1065 * t39192;
    let t39197 = t97 * t10609 * t1561;
    let t39198 = t2625 * t792;
    (t39186, t39188, t39195, t39197, t39198)
}
