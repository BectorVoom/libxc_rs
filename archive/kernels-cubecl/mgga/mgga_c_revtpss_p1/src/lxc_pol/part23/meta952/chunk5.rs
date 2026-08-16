//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3160/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3160<F: Float>(t21272: F, t5378: F, t44799: F, t82578: F, t1794: F, t5825: F, t1250: F, t1469: F, t4186: F, t12772: F, t24793: F, t3625: F) -> (F, F, F, F, F, F) {
    let t83018 = t21272 * t5378;
    let t83024 = t82578 * t44799;
    let t83033 = t5825 * t1794;
    let t83034 = t83033 * t44799;
    let t83040 = t1469 * t1794 * t1250 * t4186;
    let t83047 = t3625 * t12772 * t24793;
    (t83018, t83024, t83033, t83034, t83040, t83047)
}
