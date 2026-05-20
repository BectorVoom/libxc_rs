//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2602/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2602<F: Float>(t11007: F, t252: F, t2782: F, t6048: F, t886: F, t14481: F, t1569: F, t18805: F, t41066: F, t10995: F, t122: F, t18796: F, t2466: F) -> (F, F, F, F) {
    let t61419 = t2782 * t252 * t11007 * t6048 * t886;
    let t61422 = t2782 * t1569 * t14481;
    let t61430 = t41066 * t18805;
    let t61437 = t10995 * t18796 * t122 * t2466;
    (t61419, t61422, t61430, t61437)
}
