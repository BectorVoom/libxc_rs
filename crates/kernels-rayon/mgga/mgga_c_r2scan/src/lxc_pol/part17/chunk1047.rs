//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1047/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1047(t113: f64, t28320: f64, t3190: f64, t494: f64, t27661: f64, t560: f64, t8773: f64, t481: f64, t6212: f64, t3053: f64, t2562: f64, t2719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29951 = t28320 * t113;
    let t30007 = t3190 * t494 * t113;
    let t30049 = t27661 * t113;
    let t30053 = t8773 * t560;
    let t30057 = t8773 * t481;
    let t30119 = t6212 * t3190;
    let t30140 = t3053 * t560;
    let t30213 = t2562 * t2719;
    (t29951, t30007, t30049, t30053, t30057, t30119, t30140, t30213)
}
