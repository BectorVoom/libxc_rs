//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2615/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2615<F: Float>(t10777: F, t10779: F, t2749: F, t61956: F, t14686: F, t14931: F, t4366: F, t2661: F, t2662: F, t61625: F, t18599: F, t837: F) -> (F, F, F, F) {
    let t61959 = t10777 * t10779 * t61956 * t2749;
    let t61969 = t14931 * t14686 * t61956 * t4366;
    let t61973 = t2661 * t2662 * t61625 * t2749;
    let t61977 = t2661 * t2662 * t18599 * t837;
    (t61959, t61969, t61973, t61977)
}
