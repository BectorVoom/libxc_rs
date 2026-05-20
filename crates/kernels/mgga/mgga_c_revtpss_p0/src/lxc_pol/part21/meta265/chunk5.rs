//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1473/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1473<F: Float>(t9514: F, t9517: F, t9521: F, t9553: F, t9556: F, t9560: F, t9562: F, t9565: F, t9567: F, t9569: F, t9571: F, t9574: F) -> F {
    let t9852 = -t9553 + t9556 + t9560 + t9514 + t9562 - t9565 + t9567 - t9517 - t9521 + t9569 - t9571 - t9574;
    t9852
}
