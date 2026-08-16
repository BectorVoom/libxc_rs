//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2790/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2790<F: Float>(t10995: F, t11049: F, t14990: F, t14986: F, t2453: F, t10506: F, t2458: F, t4470: F, t10069: F, t14482: F, t15003: F, t41020: F) -> (F, F, F, F, F) {
    let t51256 = t10995 * t14990 * t11049;
    let t51258 = t2453 * t14986;
    let t51259 = t51258 * t10506;
    let t51260 = F::cast_from(0.34697458558045176417e-2_f64) * t51259;
    let t51262 = t2453 * t4470 * t2458;
    let t51263 = F::cast_from(0.34697458558045176417e-2_f64) * t51262;
    let t51264 = t10069 * t14482;
    let t51268 = t41020 * t15003;
    (t51256, t51260, t51263, t51264, t51268)
}
