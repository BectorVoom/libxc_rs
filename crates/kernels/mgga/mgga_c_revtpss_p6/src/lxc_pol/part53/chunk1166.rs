//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1166/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1166<F: Float>(t1940: F, t2255: F, t8490: F, t605: F, t7782: F, t198: F, t205: F, t8493: F, t25207: F, t4433: F, t890: F, t27383: F) -> (F, F, F, F, F, F) {
    let t126006 = t1940 * t8490 * t2255;
    let t126007 = t605 * t7782;
    let t126013 = t198 * t205 * t8493;
    let t126014 = t25207 * t4433;
    let t126017 = t7782 * t890;
    let t126018 = t27383 * t126017;
    (t126006, t126007, t126013, t126014, t126017, t126018)
}
