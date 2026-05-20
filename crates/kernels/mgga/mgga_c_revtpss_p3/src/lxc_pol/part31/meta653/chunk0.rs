//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2176/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2176<F: Float>(t25759: F, t77425: F, t100987: F, t27375: F, t106625: F, t29598: F, t94245: F, t1711: F, t4343: F, t106561: F, t27799: F, t105923: F) -> (F, F, F, F, F, F, F) {
    let t107882 = t25759 * t77425;
    let t107885 = t100987 * t27375;
    let t107892 = t25759 * t106625;
    let t107895 = t94245 * t29598;
    let t107901 = t1711 * t4343;
    let t107908 = t27799 * t106561;
    let t107919 = t25759 * t105923;
    (t107882, t107885, t107892, t107895, t107901, t107908, t107919)
}
