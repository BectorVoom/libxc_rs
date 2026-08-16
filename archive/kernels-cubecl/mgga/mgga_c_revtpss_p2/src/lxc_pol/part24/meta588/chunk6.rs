//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1842/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1842<F: Float>(t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t47014: F, t47017: F, t47020: F, t47059: F, t91966: F, t91968: F, t91969: F) -> F {
    let t92466 = t39773 - t91966 - t39783 - t39786 - t39791 - t39795 - t47014 - t91968 - t91969 + t47017 + t47020 + t39799 + t47059 + t39807 - t39813;
    t92466
}
