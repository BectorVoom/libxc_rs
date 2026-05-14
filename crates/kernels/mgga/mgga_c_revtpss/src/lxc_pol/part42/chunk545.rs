//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 545/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk545<F: Float>(t177: F, t752: F, t762: F, t717: F, t750: F, t675: F, t723: F, t169: F, t722: F, t164: F, t729: F, t730: F, t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2523 = t752 * t177;
    let t2524 = t2523 * t762;
    let t2526 = t717 * t750;
    let t2531 = t675 * t723;
    let t2535 = t722 * t169;
    let t2536 = 1.0 / t2535;
    let t2537 = t164 * t2536;
    let t2538 = t729 * t729;
    let t2539 = t2538 * t730;
    let t2548 = -0.78438333333333333333e0 * t2502 + 0.15687666666666666667e1 * t2504 + 0.68863333333333333333e0 * t2435 + 0.14025833333333333333e0 * t2509 + 0.28051666666666666667e0 * t2511 + 0.17365833333333333333e0 * t2439;
    (t2523, t2524, t2526, t2531, t2536, t2537, t2538, t2539, t2548)
}
