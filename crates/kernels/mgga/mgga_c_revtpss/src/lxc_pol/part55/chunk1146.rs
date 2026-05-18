//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1146/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1146<F: Float>(t121166: F, t25304: F, t8571: F, t121035: F, t32268: F, t32733: F, t531: F, t2411: F, t32486: F, t198: F, t206: F, t8656: F) -> (F, F, F, F, F) {
    let t121363 = t25304 * t8571 * t121166;
    let t121365 = t32268 * t121035;
    let t121593 = t531 * t32733;
    let t121716 = t32486 * t2411;
    let t121751 = t198 * t206 * t8656;
    (t121363, t121365, t121593, t121716, t121751)
}
