//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1037/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1037<F: Float>(t119955: F, t2453: F, t31778: F, t25304: F, t119813: F, t31799: F, t1032: F, t786: F, t119835: F, t119893: F, t39643: F, t8476: F) -> (F, F, F, F, F, F, F) {
    let t119957 = F::new(0.3427046870806409921e-2) * t2453 * t31778 * t119955;
    let t119960 = F::new(0.45699670022203476294e-2) * t25304 * t31778 * t119955;
    let t119966 = F::new(0.19039912555034117539e-1) * t31799 * t119813;
    let t119967 = t786 * t1032;
    let t119968 = t119967 * t119835;
    let t119969 = t119968 * t119893;
    let t119971 = t8476 * t39643;
    (t119957, t119960, t119966, t119967, t119968, t119969, t119971)
}
