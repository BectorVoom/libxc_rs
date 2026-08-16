//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1242/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1242<F: Float>(t94272: F, t94324: F, t25082: F, t49630: F, t8717: F, t530: F, t7311: F, t2014: F, t25865: F, t47672: F, t9590: F, t2034: F) -> (F, F, F, F) {
    let t94325 = t94272 + t94324;
    let t94341 = F::cast_from(9.0_f64) * t25082 * t8717 * t49630;
    let t94345 = t530 * t7311;
    let t94348 = F::cast_from(18.0_f64) * t2014 * t94345 * t25865;
    let t94349 = t47672 * t9590;
    let t94352 = F::cast_from(6.0_f64) * t2014 * t2034 * t94349;
    (t94325, t94341, t94348, t94352)
}
