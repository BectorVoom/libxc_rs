//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1105/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1105<F: Float>(t39964: F, t10710: F, t10728: F, t24902: F, t11699: F, t37939: F, t3588: F, t37932: F, t10894: F, t8243: F, t10810: F, t2184: F, t7629: F) -> (F, F, F, F, F, F) {
    let t39965 = F::cast_from(0.47609969197673950972e-2_f64) * t39964;
    let t39967 = t10728 * t10710 * t24902;
    let t39968 = F::cast_from(0.47609969197673950972e-2_f64) * t39967;
    let t39969 = t37939 * t11699;
    let t39977 = t37932 * t3588;
    let t39979 = t10894 * t8243;
    let t39980 = F::cast_from(0.10975748638225852664e-1_f64) * t39979;
    let t39982 = t2184 * t10810 * t7629;
    (t39965, t39968, t39969, t39977, t39980, t39982)
}
