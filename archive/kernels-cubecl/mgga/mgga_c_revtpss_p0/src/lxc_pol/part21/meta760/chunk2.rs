//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2686/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2686<F: Float>(t10026: F, t14141: F, t14143: F, t4056: F, t676: F, t14066: F, t1432: F, t686: F, t72: F, t14188: F, t2439: F, t2777: F) -> (F, F, F, F) {
    let t49399 = t14141 * t10026;
    let t49403 = t14141 * t14143 * t676 * t4056;
    let t49407 = t1432 * t14066 * t72 * t686;
    let t49426 = t2439 * t2777 * t14188;
    (t49399, t49403, t49407, t49426)
}
