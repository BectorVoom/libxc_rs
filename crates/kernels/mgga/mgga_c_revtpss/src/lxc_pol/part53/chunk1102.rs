//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1102/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1102<F: Float>(t119927: F, t7063: F, t119930: F, t120043: F, t31831: F, t120004: F, t25386: F, t120006: F, t2453: F, t31798: F, t119974: F, t25304: F) -> (F, F, F, F, F, F, F) {
    let t120140 = t7063 * t119927;
    let t120141 = t120140 * t119930;
    let t120149 = t31831 * t120043;
    let t120151 = t25386 * t120004;
    let t120152 = t120151 * t120006;
    let t120154 = t2453 * t31798;
    let t120156 = F::cast_from(0.95199562775170587692e-3_f64) * t120154 * t119974;
    let t120157 = t25304 * t31798;
    (t120140, t120141, t120149, t120151, t120152, t120156, t120157)
}
