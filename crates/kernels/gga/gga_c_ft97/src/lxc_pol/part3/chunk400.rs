//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 400/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk400<F: Float>(t287: F, t800: F, t194: F, t272: F, t123: F, t805: F, t289: F, t815: F) -> (F, F, F, F) {
    let t2691 = t800 * t287;
    let t2697 = F::new(1.0) / t272 / t194;
    let t2710 = t123 / t805 / t194;
    let t2724 = F::new(1.0) / t815 / t289;
    (t2691, t2697, t2710, t2724)
}
