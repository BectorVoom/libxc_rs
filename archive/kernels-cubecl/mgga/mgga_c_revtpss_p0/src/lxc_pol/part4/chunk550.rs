//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 550/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk550<F: Float>(t177: F, t752: F, t762: F, t717: F, t750: F, t675: F, t723: F, t169: F, t722: F, t164: F, t729: F, t730: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2523 = t752 * t177;
    let t2524 = t2523 * t762;
    let t2525 = F::cast_from(0.11696447245269292414e1_f64) * t2524;
    let t2526 = t717 * t750;
    let t2527 = F::cast_from(2.0_f64) * t2526;
    let t2531 = t675 * t723;
    let t2535 = t722 * t169;
    let t2536 = F::cast_from(1.0_f64) / t2535;
    let t2537 = t164 * t2536;
    let t2538 = t729 * t729;
    let t2539 = t2538 * t730;
    (t2523, t2524, t2525, t2526, t2527, t2531, t2536, t2537, t2538, t2539)
}
