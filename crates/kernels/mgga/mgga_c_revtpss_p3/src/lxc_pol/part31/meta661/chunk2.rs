//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2241/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2241<F: Float>(t2014: F, t28172: F, t28176: F, t29498: F, t94345: F, t29583: F, t7235: F, t2322: F, t30128: F, t4254: F, t1936: F, t21658: F, t651: F) -> (F, F, F, F, F, F) {
    let t109135 = F::cast_from(6.0_f64) * t2014 * t28172 * t28176;
    let t109138 = F::cast_from(6.0_f64) * t2014 * t94345 * t29498;
    let t109140 = F::cast_from(6.0_f64) * t7235 * t29583;
    let t109142 = F::cast_from(2.0_f64) * t2322 * t30128;
    let t109144 = F::cast_from(2.0_f64) * t4254 * t30128;
    let t109147 = F::cast_from(2.0_f64) * t651 * t21658 * t1936;
    (t109135, t109138, t109140, t109142, t109144, t109147)
}
