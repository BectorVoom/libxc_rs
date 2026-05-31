//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2235/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2235<F: Float>(t28182: F, t7898: F, t29499: F, t7235: F, t2014: F, t29498: F, t32737: F, t27137: F, t7732: F, t2322: F, t29502: F, t4254: F) -> (F, F, F, F, F, F) {
    let t109047 = F::cast_from(2.0_f64) * t7898 * t28182;
    let t109049 = F::cast_from(6.0_f64) * t7235 * t29499;
    let t109052 = F::cast_from(6.0_f64) * t2014 * t32737 * t29498;
    let t109054 = F::cast_from(4.0_f64) * t7732 * t27137;
    let t109058 = F::cast_from(4.0_f64) * t2322 * t29502;
    let t109060 = F::cast_from(4.0_f64) * t4254 * t29502;
    (t109047, t109049, t109052, t109054, t109058, t109060)
}
