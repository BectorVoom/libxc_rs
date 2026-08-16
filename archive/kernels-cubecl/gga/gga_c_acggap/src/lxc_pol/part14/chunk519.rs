//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 519/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk519<F: Float>(t3153: F, t3159: F, t1160: F, t1172: F, t360: F, t435: F) -> (F, F, F, F) {
    let t3160 = t3153 * t3159;
    let t3161 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t3160;
    let t3194 = t1160 * t1172;
    let t3201 = t435 * t360;
    (t3160, t3161, t3194, t3201)
}
