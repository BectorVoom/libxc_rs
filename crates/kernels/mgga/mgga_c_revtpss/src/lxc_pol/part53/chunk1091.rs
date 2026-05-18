//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1091/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1091<F: Float>(t124: F, t867: F, t14686: F, t886: F, t119836: F, t1032: F, t2735: F, t119867: F, t233: F, t240: F, t31838: F, t31840: F, t845: F) -> (F, F, F, F, F, F) {
    let t119891 = t124 * t867;
    let t119893 = t14686 * t119891 * t886;
    let t119894 = t119836 * t119893;
    let t119900 = t2735 * t1032;
    let t119903 = t119900 * t233 * t240 * t119867;
    let t119904 = F::new(0.18822977838986977999e-5) * t119903;
    let t119912 = t31838 * t845 * t31840;
    (t119891, t119893, t119894, t119900, t119904, t119912)
}
