//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1090/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1090(t37480: f64, t37523: f64, t37527: f64, t37531: f64, t37541: f64, t37560: f64, t37568: f64, t38225: f64, t38228: f64, t38233: f64, t38244: f64, t38264: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39083 = 0.26021382394247697185e-3_f64 * t37480;
    let t39091 = 0.12649025447177706166e-6_f64 * t37523;
    let t39092 = 0.89430439388620083049e-2_f64 * t37527;
    let t39093 = 0.3286404220903135089e-2_f64 * t37531;
    let t39094 = 0.487802396665200453e-2_f64 * t37541;
    let t39097 = 0.2439011983326002265e-2_f64 * t37560;
    let t39099 = 0.30487649791575028312e-3_f64 * t37568;
    let t39106 = 0.18292589874945016987e-2_f64 * t38225;
    let t39107 = 0.1299607316140891005e-4_f64 * t38228;
    let t39108 = 0.11709622077411463733e-2_f64 * t38233;
    let t39109 = 0.205201155180140685e-5_f64 * t38244;
    let t39113 = 0.30487649791575028312e-3_f64 * t38264;
    (t39083, t39091, t39092, t39093, t39094, t39097, t39099, t39106, t39107, t39108, t39109, t39113)
}
