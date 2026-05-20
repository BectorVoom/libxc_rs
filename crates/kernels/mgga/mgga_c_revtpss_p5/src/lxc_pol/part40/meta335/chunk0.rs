//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1123/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1123<F: Float>(t1235: F, t12984: F, t12627: F, t225: F, t127: F, t3672: F, t371: F, t3671: F, t140: F, t3693: F, t1222: F, t1226: F, t697: F) -> (F, F, F, F, F) {
    let t12985 = t1235 * t12984;
    let t12987 = t12627 * t225;
    let t12995 = t371 * t127 * t3672;
    let t12996 = t3671 * t12995;
    let t12998 = t140 * t3693;
    let t12999 = t1222 * t12998;
    let t13011 = t697 * t1226;
    (t12985, t12987, t12996, t12999, t13011)
}
