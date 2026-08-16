//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 869/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk869<F: Float>(t1994: F, t30179: F, t1039: F, t1997: F, t3055: F, t1967: F, t7784: F, t1200: F, t7614: F, t30169: F, t601: F, t3646: F, t597: F) -> (F, F, F, F, F, F) {
    let t30180 = t30179 * t1994;
    let t30183 = t3055 * t1997 * t1039;
    let t30185 = t1967 * t7784;
    let t30187 = t7614 * t1200;
    let t30191 = t30169 * t601;
    let t30193 = t3646 * t597;
    (t30180, t30183, t30185, t30187, t30191, t30193)
}
