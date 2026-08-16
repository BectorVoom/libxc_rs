//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1985/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1985<F: Float>(t13790: F, t5658: F, t1398: F, t23037: F, t543: F, t74700: F, t116: F, t21813: F, t5966: F, t890: F, t5962: F, t1544: F, t4537: F) -> (F, F, F, F, F, F, F) {
    let t75188 = t13790 * t5658;
    let t75267 = t23037 * t1398;
    let t75305 = t74700 * t543;
    let t75439 = t21813 * t116;
    let t77408 = t5966 * t890;
    let t77425 = t5962 * t890;
    let t77441 = t1544 * t4537;
    (t75188, t75267, t75305, t75439, t77408, t77425, t77441)
}
