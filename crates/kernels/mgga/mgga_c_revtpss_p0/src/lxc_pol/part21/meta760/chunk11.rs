//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2695/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2695<F: Float>(t1353: F, t4144: F, t14304: F, t4147: F, t13674: F, t13872: F, t1448: F, t39528: F, t39531: F, t4139: F, t4140: F, t48228: F, t48231: F, t48232: F, t48234: F, t48236: F, t48238: F, t5536: F, t5541: F) -> F {
    let t49560 = t4144 * t1353;
    let t49564 = t14304 * t4147;
    let t49571 = F::new(18.0) * t13674 * t4139 * t49560 + F::new(18.0) * t13872 * t4140 * t5536 - F::new(3.0) * t1448 * t49564 * t5541 - t39528 + t39531 + t48228 + t48231 - t48232 - t48234 + t48236 + t48238;
    t49571
}
