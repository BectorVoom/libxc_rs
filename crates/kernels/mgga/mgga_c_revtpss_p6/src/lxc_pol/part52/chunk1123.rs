//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1123/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1123<F: Float>(t119822: F, t25386: F, t2670: F, t31831: F, t119839: F, t119968: F, t31805: F, t860: F, t817: F, t8485: F, t240: F, t27: F, t822: F) -> (F, F, F, F, F, F) {
    let t119982 = t25386 * t119822;
    let t119989 = t31831 * t2670;
    let t119991 = t119968 * t119839;
    let t119992 = F::cast_from(0.150583822711895824e-3_f64) * t119991;
    let t120000 = t31805 * t860;
    let t120002 = t120000 * t8485 * t817;
    let t120003 = F::cast_from(0.66119071333692697238e-4_f64) * t120002;
    let t120010 = t822 * t27 * t240;
    (t119982, t119989, t119992, t120000, t120003, t120010)
}
