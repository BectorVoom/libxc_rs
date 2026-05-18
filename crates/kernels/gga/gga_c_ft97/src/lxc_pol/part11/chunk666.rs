//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 666/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk666<F: Float>(t9204: F, t9242: F, t579: F, t91: F, t153: F, t525: F, t631: F, t637: F, t7242: F, t2087: F, t590: F, t9014: F, t9024: F, t9028: F, t9032: F, t9057: F, t9059: F, t9062: F, t9076: F, t9080: F, t9166: F, t9170: F) -> (F, F, F, F, F) {
    let t9243 = t9204 + t9242;
    let t9245 = t91 * t579 * t9243;
    let t9252 = F::new(1.0) / t153 / t631 / t637 / t525 / t7242 / F::new(4.0);
    let t9253 = t2087 * t590;
    let t9255 = t91 * t9252 * t9253;
    let t9257 = -F::new(2.0) / F::new(3.0) * t9059 - F::new(2.0) * t9076 - F::new(2.0) * t9080 - t9166 - t9014 / F::new(3.0) - F::new(3.0) / F::new(4.0) * t9170 + F::new(6.0) * t9024 - F::new(10.0) / F::new(27.0) * t9028 - F::new(2.0) * t9032 + F::new(4.0) / F::new(3.0) * t9057 - F::new(2.0) / F::new(3.0) * t9062 + t9245 / F::new(2.0) + F::new(3.0) / F::new(8.0) * t9255;
    (t9243, t9245, t9252, t9255, t9257)
}
