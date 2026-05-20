//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2178/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2178<F: Float>(t11064: F, t1711: F, t27384: F, t106533: F, t25759: F, t100987: F, t18875: F, t4433: F, t892: F, t1113: F, t5962: F, t18392: F, t33: F) -> (F, F, F, F, F, F) {
    let t107923 = t11064 * t1711;
    let t107924 = t107923 * t27384;
    let t107927 = t25759 * t106533;
    let t107930 = t100987 * t18875;
    let t107934 = t892 * t1711 * t4433;
    let t107939 = t1113 * t5962;
    let t107943 = t33 * t18392;
    (t107924, t107927, t107930, t107934, t107939, t107943)
}
