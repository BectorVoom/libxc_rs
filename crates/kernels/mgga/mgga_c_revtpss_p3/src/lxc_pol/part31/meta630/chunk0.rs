//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2084/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2084<F: Float>(t25569: F, t4817: F, t1659: F, t25576: F, t27489: F, t3111: F, t11940: F, t7131: F, t16158: F, t7132: F, t100007: F, t16094: F) -> (F, F, F, F, F, F) {
    let t100097 = F::cast_from(0.3811023832717309953e-3_f64) * t25569 * t4817;
    let t100114 = t1659 * t25576;
    let t100117 = t27489 * t3111;
    let t100121 = t11940 * t7131;
    let t100132 = F::cast_from(0.3811023832717309953e-3_f64) * t7132 * t16158;
    let t100135 = t16094 * t100007;
    (t100097, t100114, t100117, t100121, t100132, t100135)
}
