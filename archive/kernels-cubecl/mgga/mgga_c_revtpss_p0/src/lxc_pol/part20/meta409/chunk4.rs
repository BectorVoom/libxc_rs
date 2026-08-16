//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1517/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1517<F: Float>(t41573: F, t41577: F, t41580: F, t41582: F, t41585: F, t41591: F, t41657: F, t41841: F, t41845: F, t41847: F, t41849: F, t41933: F) -> F {
    let t42849 = t41573 + t41577 + t41580 + t41582 + t41585 - t41591 + t41657 + t41933 + t41841 + t41845 - t41847 + t41849;
    t42849
}
