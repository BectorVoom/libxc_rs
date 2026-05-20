//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1285/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1285<F: Float>(t128274: F, t128277: F, t128279: F, t128280: F, t128282: F, t128284: F, t128287: F, t128289: F, t128293: F, t128294: F, t128295: F, t128301: F, t28653: F, t33287: F, t4293: F, t7591: F) -> F {
    let t130961 = -F::new(2.0) * t28653 * t7591 - F::new(2.0) * t33287 * t4293 - t128274 + t128277 + t128279 - t128280 + t128282 - t128284 - t128287 - F::new(2.0) * t128289 - F::new(2.0) * t128293 - F::new(2.0) * t128294 - F::new(2.0) * t128295 - F::new(2.0) * t128301;
    t130961
}
