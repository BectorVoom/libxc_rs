//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 316/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk316<F: Float>(t3653: F, t637: F, t639: F, t2251: F, t2254: F, t2256: F, t2265: F, t3611: F, t3614: F, t3618: F, t3622: F, t3628: F, t3630: F, t3633: F, t3637: F, t3642: F, t631: F) -> (F,) {
    let t3655 = t637 * t639 * t3653;
    let t3658 = -t2251 - t2254 / 9.0 - t2256 / 3.0 - t3611 / 9.0 + t2265 * t3614 / 18.0 - t2265 * t3618 / 3.0 - t2265 * t3622 / 3.0 + t3628 * t3630 / 3.0 - t3633 / 3.0 - t2265 * t3637 / 3.0 - 3.0 / 2.0 * t631 * t3642 + t631 * t3655 / 2.0;
    (t3658,)
}
