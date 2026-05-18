//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1075/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1075<F: Float>(t2439: F, t785: F, t8578: F, t8580: F, t121210: F, t2453: F, t8705: F, t25304: F, t32237: F, t121142: F, t596: F, t8571: F) -> (F, F, F, F, F) {
    let t121259 = F::new(0.4818682326780666368e-3) * t2439 * t785 * t8578 * t8580;
    let t121272 = t2453 * t8705 * t121210;
    let t121273 = F::new(0.3718732920905101082e-5) * t121272;
    let t121275 = t25304 * t8705 * t121210;
    let t121276 = F::new(0.19835721400107809171e-4) * t121275;
    let t121285 = t2453 * t32237;
    let t121287 = F::new(0.95199562775170587692e-3) * t121285 * t121142;
    let t121305 = t8571 * t596;
    (t121259, t121273, t121276, t121287, t121305)
}
