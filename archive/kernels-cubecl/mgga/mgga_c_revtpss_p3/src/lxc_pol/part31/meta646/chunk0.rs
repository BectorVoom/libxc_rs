//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2112/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2112<F: Float>(t18414: F, t2661: F, t93082: F, t18418: F, t25227: F, t18398: F, t7045: F, t18402: F, t25234: F, t18409: F, t25266: F, t5980: F) -> (F, F, F, F, F, F) {
    let t106030 = t2661 * t93082 * t18414;
    let t106033 = t2661 * t25227 * t18418;
    let t106035 = t7045 * t18398;
    let t106037 = t25234 * t18402;
    let t106040 = t2661 * t25227 * t18409;
    let t106042 = t25266 * t5980;
    (t106030, t106033, t106035, t106037, t106040, t106042)
}
