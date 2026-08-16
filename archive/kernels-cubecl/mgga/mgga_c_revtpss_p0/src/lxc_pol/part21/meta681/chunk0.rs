//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2494/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2494<F: Float>(t12800: F, t3636: F, t3551: F, t3565: F, t225: F, t12884: F, t828: F, t12788: F, t3625: F, t12732: F, t73: F, t3555: F, t3766: F, t5330: F) -> (F, F, F, F, F, F, F) {
    let t44418 = t12800 * t3636;
    let t44420 = t3551 * t3565;
    let t44421 = t44420 * t225;
    let t44425 = t828 * t12884;
    let t44427 = t3625 * t44425 * t12788;
    let t44431 = t12732 * t73;
    let t44484 = t3555 * t3766 * t5330;
    (t44418, t44420, t44421, t44425, t44427, t44431, t44484)
}
