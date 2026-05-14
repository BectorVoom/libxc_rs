//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 415/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk415<F: Float>(t2568: F, t5064: F, t242: F, t2574: F, t265: F, t4934: F, t1131: F, t1168: F) -> (F, F, F) {
    let t5065 = t2568 * t5064;
    let t5066 = t242 * t5065;
    let t5070 = t2574 * t265 * t4934;
    let t5073 = t1131 * t1168;
    (t5066, t5070, t5073)
}
