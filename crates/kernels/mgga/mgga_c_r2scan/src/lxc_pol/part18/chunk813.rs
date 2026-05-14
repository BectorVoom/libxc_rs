//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 813/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk813<F: Float>(t3216: F, t788: F, t2201: F, t785: F, t3190: F, t481: F, t551: F, t552: F, t113: F, t8820: F) -> (F, F, F) {
    let t9434 = t788 * t3216;
    let t9436 = t2201 * t785 * t9434;
    let t9439 = t3190 * t481;
    let t9441 = t551 * t552 * t9439;
    let t9445 = t8820 * t113;
    (t9436, t9441, t9445)
}
