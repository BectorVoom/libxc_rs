//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3250/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3250<F: Float>(t10317: F, t10328: F, t10331: F, t10336: F, t13343: F, t13346: F, t13389: F, t1494: F, t2258: F, t2259: F, t2260: F, t2263: F, t2312: F, t4196: F, t4217: F, t4238: F, t608: F, t641: F, t7719: F, t85: F) -> F {
    let t60417 = -t2259 * t4217 * t85 / F::new(4.0) - t13343 * t641 / F::new(4.0) - t13346 * t641 / F::new(2.0) - t4196 * t2312 / F::new(4.0) - t10317 * t7719 * t2258 / F::new(4.0) - t10328 * t1494 / F::new(12.0) - t10331 * t1494 / F::new(4.0) - t2260 * t4238 / F::new(4.0) - t10336 * t1494 / F::new(4.0) - t2263 * t4238 / F::new(2.0) - t608 * t13389 / F::new(4.0);
    t60417
}
