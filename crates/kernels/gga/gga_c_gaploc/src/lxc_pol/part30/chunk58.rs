//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 58/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk58<F: Float>(t2: F, t140: F, t145: F) -> (F, F, F, F) {
    let t146 = F::sqrt(F::cast_from(3.0_f64));
    let t148 = F::sqrt(t2);
    let t149 = F::cast_from(1.0_f64) / t148;
    let t151 = t140 * t145 * t146 * t149;
    let t153 = F::cast_from(0.854613e1_f64) + t151 / F::cast_from(64.0_f64);
    (t146, t149, t151, t153)
}
