//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 898/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk898<F: Float>(t28280: F, t572: F, t1461: F, t1918: F, t2040: F, t28246: F, t28257: F, t28259: F, t28261: F, t28263: F, t28267: F, t28270: F, t28273: F, t28275: F, t28279: F, t573: F, t5802: F, t5805: F, t7324: F, t7944: F) -> F {
    let t28282 = F::new(3.0) * t572 * t28280;
    let t28283 = F::new(3.0) * t1461 * t7944 + F::new(3.0) * t1918 * t7324 + F::new(6.0) * t2040 * t5802 + F::new(3.0) * t2040 * t5805 + t28246 * t573 + t28257 + t28259 + t28261 + t28263 + t28267 + t28270 + t28273 + t28275 + t28279 + t28282;
    t28283
}
