//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 895/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk895<F: Float>(t144: F, t34948: F, t34950: F, t1060: F, t574: F, t7339: F, t28: F, t33155: F, t33161: F, t35118: F, t35122: F, t35127: F, t35151: F, t35157: F, t35162: F, t35166: F, t446: F, t89: F) -> (F, F, F, F) {
    let t35169 = t144 * t34948;
    let t35172 = t144 * t34950;
    let t35176 = t574 * t1060 * t7339;
    let t35179 = -t33155 - F::new(2.0) / F::new(3.0) * t446 * t35118 + F::new(2.0) / F::new(3.0) * t446 * t35122 + t446 * t35127 / F::new(3.0) + t89 * t28 * t35151 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t35157 + F::new(2.0) / F::new(3.0) * t446 * t35162 - t446 * t35166 / F::new(3.0) - t33161 - t446 * t35169 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t35172 - t446 * t35176 / F::new(3.0);
    (t35169, t35172, t35176, t35179)
}
