//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 966/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk966<F: Float>(t1042: F, t24639: F, t23842: F, t5302: F, t1774: F, t5825: F, t5296: F, t24244: F, t5308: F, t24236: F, t5312: F, t13046: F, t24544: F, t13053: F, t1803: F, t6601: F) -> (F, F, F, F, F, F, F, F) {
    let t24640 = t1042 * t24639;
    let t24643 = t5302 * t23842;
    let t24644 = t1042 * t24643;
    let t24647 = t5825 * t1774;
    let t24648 = t5296 * t24647;
    let t24649 = t1042 * t24648;
    let t24652 = t5308 * t24244;
    let t24655 = t5312 * t24236;
    let t24663 = t24544 * t13046;
    let t24664 = t1042 * t24663;
    let t24667 = t24544 * t13053;
    let t24668 = t1042 * t24667;
    let t24671 = t6601 * t1803;
    (t24640, t24644, t24649, t24652, t24655, t24664, t24668, t24671)
}
