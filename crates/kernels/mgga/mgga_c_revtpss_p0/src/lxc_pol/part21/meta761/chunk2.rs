//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2698/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2698<F: Float>(t1353: F, t13716: F, t4139: F, t4140: F, t47076: F, t48281: F, t48283: F, t48284: F, t48286: F, t48288: F, t48291: F, t48293: F, t48295: F, t5536: F, t566: F) -> F {
    let t49611 = F::new(18.0) * t1353 * t13716 * t5536 * t566 + F::new(9.0) * t13716 * t4139 * t4140 - t47076 - t48281 - t48283 - t48284 + t48286 + t48288 - t48291 + t48293 - t48295;
    t49611
}
