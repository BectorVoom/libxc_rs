//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1045/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1045<F: Float>(t119868: F, t2453: F, t8464: F, t817: F, t8485: F, t93341: F, t119927: F, t7063: F, t119930: F, t120043: F, t31831: F, t120004: F, t25386: F) -> (F, F, F, F, F, F) {
    let t120132 = t2453 * t8464 * t119868;
    let t120133 = F::cast_from(0.13386901839087538753e-4_f64) * t120132;
    let t120138 = t93341 * t8485 * t817;
    let t120140 = t7063 * t119927;
    let t120141 = t120140 * t119930;
    let t120149 = t31831 * t120043;
    let t120151 = t25386 * t120004;
    (t120133, t120138, t120140, t120141, t120149, t120151)
}
