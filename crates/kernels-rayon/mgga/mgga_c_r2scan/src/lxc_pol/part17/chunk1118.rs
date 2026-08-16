//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1118/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1118(t10899: f64, t11770: f64, t2201: f64, t2834: f64, t3316: f64, t20407: f64, t2161: f64, t2841: f64, t625: f64, t37982: f64, t7620: f64, t10856: f64, t7407: f64) -> (f64, f64, f64, f64, f64) {
    let t40220 = t2201 * t10899 * t11770;
    let t40222 = t2834 * t3316;
    let t40228 = t2161 * t20407 * t2841 * t625;
    let t40232 = t37982 * t7620;
    let t40234 = t10856 * t7407;
    (t40220, t40222, t40228, t40232, t40234)
}
