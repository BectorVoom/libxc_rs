//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2620/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2620<F: Float>(t10811: F, t18639: F, t10905: F, t18507: F, t10777: F, t10779: F, t2749: F, t61715: F, t18651: F, t14923: F, t18456: F, t14671: F, t14686: F, t14931: F, t18632: F) -> (F, F, F, F, F, F) {
    let t62162 = t10811 * t18639;
    let t62168 = t10905 * t18507;
    let t62176 = t10777 * t10779 * t61715 * t2749;
    let t62178 = t10811 * t18651;
    let t62188 = t14923 * t18456;
    let t62216 = t14931 * t14686 * t14671 * t18632;
    (t62162, t62168, t62176, t62178, t62188, t62216)
}
