//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1018/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1018<F: Float>(t121166: F, t25304: F, t8571: F, t121035: F, t32268: F, t1455: F, t8734: F, t32733: F, t531: F, t32151: F, t32597: F, t10301: F, t32589: F, t136: F, t2247: F, t26178: F) -> (F, F, F, F, F, F, F) {
    let t121363 = t25304 * t8571 * t121166;
    let t121365 = t32268 * t121035;
    let t121531 = t1455 * t8734;
    let t121593 = t531 * t32733;
    let t121617 = t32597 * t32151;
    let t121625 = t10301 * t32589;
    let t121629 = t2247 * t26178 * t136;
    (t121363, t121365, t121531, t121593, t121617, t121625, t121629)
}
