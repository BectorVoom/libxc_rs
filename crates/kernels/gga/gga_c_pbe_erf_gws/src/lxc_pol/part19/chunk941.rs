//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 941/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk941<F: Float>(t10201: F, t247: F, t251: F, t3583: F, t719: F, t256: F, t19: F, t3379: F, t336: F, t714: F, t1046: F, t2816: F) -> (F, F, F, F) {
    let t10602 = t10201 * t247;
    let t10603 = t10602 * t251;
    let t10606 = t3583 * t719;
    let t10607 = t10606 * t256;
    let t10609 = t3379 * t19;
    let t10610 = t10609 * t336;
    let t10611 = t10610 * t714;
    let t10614 = F::new(4.0) / F::new(15.0) * t2816 * t1046;
    (t10603, t10607, t10611, t10614)
}
