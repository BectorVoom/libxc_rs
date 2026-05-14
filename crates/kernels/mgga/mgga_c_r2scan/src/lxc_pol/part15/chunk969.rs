//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 969/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk969<F: Float>(t261: F, t3299: F, t6507: F, t3304: F, t6503: F, t10872: F, t10885: F, t1582: F, t2096: F, t571: F, t10769: F, t3281: F, t6245: F, t120: F, t6511: F, t531: F) -> (F, F, F, F, F, F, F, F) {
    let t37848 = t3299 * t261 * t6507;
    let t37851 = t3304 * t261 * t6503;
    let t37859 = t10872 * t10885;
    let t37880 = t571 * t1582 * t2096;
    let t37881 = t37880 * t10769;
    let t37883 = t3281 * t6245;
    let t37890 = t120 * t6511;
    let t37891 = t37890 * t531;
    (t37848, t37851, t37859, t37880, t37881, t37883, t37890, t37891)
}
