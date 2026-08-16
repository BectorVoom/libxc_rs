//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 815/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk815<F: Float>(t34: F, t649: F, t641: F, t837: F, t2592: F, t639: F, t2597: F, t5493: F, t1620: F, t1627: F, t2640: F, t1660: F, t197: F) -> (F, F, F, F, F) {
    let t7019 = t649 * t34;
    let t7039 = t837 * t641;
    let t7040 = t7039 * t2592;
    let t7041 = t639 * t7040;
    let t7043 = t5493 * t2597;
    let t7045 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1620 * t7043;
    let t7047 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t1627 * t2640;
    let t7048 = t1660 * t197;
    (t7019, t7041, t7045, t7047, t7048)
}
