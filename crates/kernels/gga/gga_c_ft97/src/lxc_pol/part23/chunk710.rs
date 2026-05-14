//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 710/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk710<F: Float>(t5147: F, t713: F, t729: F, t762: F, t766: F, t2568: F, t242: F, t18: F, t992: F) -> (F, F, F, F, F) {
    let t18486 = t5147 * t713;
    let t18488 = t729 * t762 * t18486;
    let t18491 = t5147 * t766;
    let t18492 = t2568 * t18491;
    let t18493 = t242 * t18492;
    let t18497 = t992 * t18;
    (t18486, t18488, t18491, t18493, t18497)
}
