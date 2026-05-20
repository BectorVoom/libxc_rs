//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3282/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3282<F: Float>(t40103: F, t40115: F, t40131: F, t40137: F, t61200: F, t61202: F, t61209: F, t61214: F, t61215: F, t61219: F, t61220: F, t61222: F, t61224: F, t61225: F, t61229: F, t61240: F, t61244: F, t61245: F, t61248: F) -> F {
    let t62266 = t40103 + t61200 + t61202 - t40115 + t61209 + t61214 + t61215 + t61219 - t40131 - t61220 - t40137 + t61222 + t61224 + t61225 + t61229 - t61240 + t61244 + t61245 + t61248;
    t62266
}
