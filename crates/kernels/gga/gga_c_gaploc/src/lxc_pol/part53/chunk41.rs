//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 41/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk41<F: Float>(t2: F, t140: F, t145: F) -> (F, F, F, F) {
    let t146 = F::sqrt(F::new(3.0));
    let t148 = F::sqrt(t2);
    let t149 = F::new(1.0) / t148;
    let t151 = t140 * t145 * t146 * t149;
    let t153 = F::new(0.854613e1) + t151 / F::new(64.0);
    (t146, t149, t151, t153)
}
