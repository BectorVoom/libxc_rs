//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 458/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk458<F: Float>(t2457: F, t2501: F, t2470: F, t684: F, t128: F, t136: F, t692: F, t2435: F, t2439: F, t738: F, t745: F, t760: F) -> (F, F, F, F, F, F, F) {
    let t2502 = t2501 * t2457;
    let t2504 = t684 * t2470;
    let t2507 = F::new(1.0)/f64::sqrt(t128);
    let t2508 = t2507 * t136;
    let t2509 = t2508 * t2457;
    let t2511 = t692 * t2470;
    let t2514 = -F::new(0.57538888888888888889e0) * t2502 + F::new(0.11507777777777777778e1) * t2504 + F::new(0.40256666666666666667e0) * t2435 + F::new(0.366775e-1) * t2509 + F::new(0.73355e-1) * t2511 + F::new(0.137975e0) * t2439;
    let t2516 = t738 * t2514 * t745;
    let t2518 = F::new(0.5848223622634646207e0) * t760 * t2516;
    (t2502, t2504, t2509, t2511, t2514, t2516, t2518)
}
