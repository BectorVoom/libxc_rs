//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 215/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk215<F: Float>(t710: F, t35: F, t39: F, t88: F, t223: F, t228: F, t4: F, t6: F) -> (F, F, F, F, F, F) {
    let t711 = F::cast_from(20.0_f64) * t710;
    let t712 = t35 * t39;
    let t713 = t712 * t88;
    let t714 = F::cast_from(12.0_f64) * t713;
    let t715 = t223 * t228;
    let t716 = t715 * t88;
    let t717 = F::cast_from(32.0_f64) * t716;
    let t721 = t4 * t6;
    (t711, t712, t714, t715, t717, t721)
}
