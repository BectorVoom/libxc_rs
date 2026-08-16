//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 863/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk863<F: Float>(t377: F, t7779: F, t606: F, t7: F, t7508: F, t8: F, t151: F, t56: F, t593: F, t1994: F, t1039: F, t1997: F, t3055: F) -> (F, F, F, F, F, F) {
    let t30169 = t377 * t7779;
    let t30170 = t30169 * t606;
    let t30171 = F::cast_from(0.19812298142450615803e-1_f64) * t30170;
    let t30174 = t7508 * t7;
    let t30176 = F::cast_from(1.0_f64) / t8 / t30174;
    let t30179 = t151 * t593 * t30176 * t56;
    let t30180 = t30179 * t1994;
    let t30181 = F::cast_from(0.16050174509286859832e-1_f64) * t30180;
    let t30183 = t3055 * t1997 * t1039;
    (t30169, t30171, t30174, t30179, t30181, t30183)
}
