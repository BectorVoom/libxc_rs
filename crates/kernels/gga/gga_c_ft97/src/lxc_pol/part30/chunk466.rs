//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 466/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk466<F: Float>(t1466: F, t1479: F, t301: F, t7581: F, t7587: F, t7614: F, t7618: F, t7668: F, t7673: F, t7680: F, t7684: F, t7686: F, t332: F, t12: F, t52: F) -> (F, F, F) {
    let t7691 = t7581 * t1479 / 6.0 - t1466 * t7587 / 3.0 + t1466 * t7614 / 6.0 + t1466 * t7618 / 3.0 - t301 * t7684 + 2.0 * t7686 - 4.0 * t7668 + 4.0 * t7673 - 2.0 * t7680;
    let t7692 = t7691 * t332;
    let t7853 = t52 * t12;
    (t7691, t7692, t7853)
}
