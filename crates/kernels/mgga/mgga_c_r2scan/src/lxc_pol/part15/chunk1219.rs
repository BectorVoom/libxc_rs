//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1219/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1219<F: Float>(t32212: F, t481: F, t14160: F, t40630: F, t11550: F, t792: F, t3262: F, t3276: F, t10648: F, t10971: F, t11564: F, t10610: F, t3263: F) -> (F, F, F, F) {
    let t40631 = t32212 * t481;
    let t40634 = F::cast_from(3.0_f64) * t40630 * t14160 * t40631;
    let t40635 = t11550 * t792;
    let t40638 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t3276 * t40635;
    let t40642 = t10648 * t10971 * t11564;
    let t40644 = t11550 * t481;
    let t40647 = F::cast_from(3.0_f64) * t10610 * t3263 * t40644;
    (t40634, t40638, t40642, t40647)
}
