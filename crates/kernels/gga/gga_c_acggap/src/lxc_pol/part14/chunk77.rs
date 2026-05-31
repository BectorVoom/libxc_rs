//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 77/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk77<F: Float>(t195: F, t31: F, t4: F, t27: F, t13: F, t1: F, t137: F, t3: F, t6: F) -> (F, F, F, F, F, F) {
    let t197 = t4 * t195 * t31;
    let t198 = F::cast_from(0.11073470983333333333e-2_f64) * t197;
    let t199 = t27 * t27;
    let t200 = F::cast_from(1.0_f64) / t199;
    let t201 = t13 * t200;
    let t202 = t137 * t1;
    let t203 = t3 * t6;
    (t198, t199, t200, t201, t202, t203)
}
