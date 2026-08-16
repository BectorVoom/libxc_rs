//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2763/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2763<F: Float>(t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t49979: F, t49982: F, t49984: F, t49987: F, t49992: F, t49994: F, t49995: F, t50037: F) -> F {
    let t50848 = -t39783 - t39786 - t39791 - t39795 + t49979 + t49982 + t49984 - t49987 + t39799 + t39807 - t39813 - t39818 - t39823 + t49992 + t49994 - t49995 + t50037;
    t50848
}
