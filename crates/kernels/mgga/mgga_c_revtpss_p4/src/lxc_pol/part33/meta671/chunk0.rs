//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2199/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2199<F: Float>(t18227: F, t7742: F, t28063: F, t4248: F, t28182: F, t7898: F, t29499: F, t7235: F, t2014: F, t29498: F, t32737: F, t27137: F, t7732: F) -> (F, F, F, F, F, F) {
    let t109043 = F::new(4.0) * t18227 * t7742;
    let t109045 = F::new(4.0) * t4248 * t28063;
    let t109047 = F::new(2.0) * t7898 * t28182;
    let t109049 = F::new(6.0) * t7235 * t29499;
    let t109052 = F::new(6.0) * t2014 * t32737 * t29498;
    let t109054 = F::new(4.0) * t7732 * t27137;
    (t109043, t109045, t109047, t109049, t109052, t109054)
}
