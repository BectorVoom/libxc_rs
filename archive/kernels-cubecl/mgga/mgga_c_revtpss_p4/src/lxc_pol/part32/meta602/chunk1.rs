//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1938/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1938<F: Float>(t18398: F, t7045: F, t18402: F, t25234: F, t18409: F, t25227: F, t2661: F, t25266: F, t5980: F, t18482: F, t25270: F, t18478: F, t27261: F) -> (F, F, F, F, F, F) {
    let t106035 = t7045 * t18398;
    let t106037 = t25234 * t18402;
    let t106040 = t2661 * t25227 * t18409;
    let t106042 = t25266 * t5980;
    let t106044 = t25270 * t18482;
    let t106046 = t27261 * t18478;
    (t106035, t106037, t106040, t106042, t106044, t106046)
}
