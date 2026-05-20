//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3036/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3036<F: Float>(t252: F, t2769: F, t2782: F, t4533: F, t886: F, t10995: F, t11049: F, t14990: F, t14986: F, t2453: F, t10506: F, t2458: F, t4470: F) -> (F, F, F, F, F) {
    let t51251 = t2782 * t252 * t2769 * t4533 * t886;
    let t51256 = t10995 * t14990 * t11049;
    let t51258 = t2453 * t14986;
    let t51259 = t51258 * t10506;
    let t51262 = t2453 * t4470 * t2458;
    (t51251, t51256, t51258, t51259, t51262)
}
