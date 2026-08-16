//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1117/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1117(t29418: f64, t3293: f64, t132: f64, t537: f64, t1575: f64, t25826: f64, t3342: f64, t571: f64, t10856: f64, t8071: f64, t37769: f64, t7620: f64) -> (f64, f64, f64, f64, f64) {
    let t40194 = t3293 * t29418;
    let t40195 = t132 * t537;
    let t40201 = t571 * t1575 * t3342 * t25826;
    let t40215 = t10856 * t8071;
    let t40217 = t37769 * t7620;
    (t40194, t40195, t40201, t40215, t40217)
}
