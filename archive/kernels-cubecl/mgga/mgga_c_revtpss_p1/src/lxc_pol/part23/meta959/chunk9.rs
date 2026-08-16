//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3230/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3230<F: Float>(t1300: F, t198: F, t336: F, t81646: F, t81649: F, t81653: F, t81656: F, t81660: F, t82119: F, t82169: F, t82220: F, t82266: F, t82391: F, t82394: F, t82396: F, t82398: F, t84241: F, t84290: F, t84337: F, t84947: F, t84992: F) -> F {
    let t84999 = -t81646 - t81649 + t81653 + t81656 + t81660 + t82119 + t198 * t336 * (t82169 + t82220 + t82266 + t84241 + t84290 + t84337 + t84947 + t84992) * t1300 - t82391 - t82394 - t82396 - t82398;
    t84999
}
