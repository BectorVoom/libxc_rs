//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1105/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1105(t39964: f64, t10710: f64, t10728: f64, t24902: f64, t11699: f64, t37939: f64, t3588: f64, t37932: f64, t10894: f64, t8243: f64, t10810: f64, t2184: f64, t7629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39965 = 0.47609969197673950972e-2_f64 * t39964;
    let t39967 = t10728 * t10710 * t24902;
    let t39968 = 0.47609969197673950972e-2_f64 * t39967;
    let t39969 = t37939 * t11699;
    let t39977 = t37932 * t3588;
    let t39979 = t10894 * t8243;
    let t39980 = 0.10975748638225852664e-1_f64 * t39979;
    let t39982 = t2184 * t10810 * t7629;
    (t39965, t39968, t39969, t39977, t39980, t39982)
}
