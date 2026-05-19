//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 864/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk864<F: Float>(t1994: F, t30179: F, t1039: F, t1997: F, t3055: F, t1967: F, t7784: F, t1200: F, t7614: F, t30169: F, t601: F, t3646: F, t597: F) -> (F, F, F, F, F, F) {
    let t30180 = t30179 * t1994;
    let t30181 = F::cast_from(0.16050174509286859832e-1_f64) * t30180;
    let t30183 = t3055 * t1997 * t1039;
    let t30184 = F::cast_from(0.25724410870841842183e-2_f64) * t30183;
    let t30185 = t1967 * t7784;
    let t30187 = t7614 * t1200;
    let t30191 = t30169 * t601;
    let t30192 = F::cast_from(0.13505315707191967146e-1_f64) * t30191;
    let t30193 = t3646 * t597;
    (t30181, t30184, t30185, t30187, t30192, t30193)
}
