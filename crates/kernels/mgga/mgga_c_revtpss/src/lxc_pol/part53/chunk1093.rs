//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1093/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1093<F: Float>(t119934: F, t31752: F, t31758: F, t119857: F, t1955: F, t136: F, t233: F, t2457: F, t2453: F, t31778: F, t25304: F, t119813: F, t31799: F) -> (F, F, F, F, F, F) {
    let t119935 = t31752 * t119934;
    let t119936 = t119935 * t31758;
    let t119941 = t1955 * t119857;
    let t119955 = t233 * t136 * t2457;
    let t119957 = F::new(0.3427046870806409921e-2) * t2453 * t31778 * t119955;
    let t119960 = F::new(0.45699670022203476294e-2) * t25304 * t31778 * t119955;
    let t119966 = F::new(0.19039912555034117539e-1) * t31799 * t119813;
    (t119935, t119936, t119941, t119957, t119960, t119966)
}
