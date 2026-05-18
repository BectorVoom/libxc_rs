//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1033/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1033<F: Float>(t119971: F, t31798: F, t136: F, t2457: F, t8480: F, t119822: F, t25386: F, t119826: F, t119830: F, t32469: F, t2670: F, t31831: F) -> (F, F, F, F, F, F) {
    let t119972 = t119971 * t31798;
    let t119974 = t8480 * t136 * t2457;
    let t119976 = F::new(0.6019057092162847523e-2) * t119972 * t119974;
    let t119982 = t25386 * t119822;
    let t119983 = t119982 * t119826;
    let t119985 = t32469 * t119830;
    let t119989 = t31831 * t2670;
    (t119974, t119976, t119982, t119983, t119985, t119989)
}
