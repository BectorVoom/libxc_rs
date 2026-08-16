//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 889/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk889<F: Float>(t3375: F, t3427: F, t1020: F, t3670: F, t12273: F, t150: F, t3213: F, t383: F, t1039: F, t3055: F, t996: F, t3178: F) -> (F, F, F, F, F, F) {
    let t13192 = t3375 * t3427;
    let t13221 = t3670 * t1020;
    let t13223 = t12273 * t150;
    let t13226 = F::cast_from(0.51448821741683684368e-2_f64) * t13223 * t383 * t3213;
    let t13229 = F::cast_from(0.24009450146119052704e-1_f64) * t3055 * t996 * t1039;
    let t13230 = t3375 * t3178;
    (t13192, t13221, t13223, t13226, t13229, t13230)
}
