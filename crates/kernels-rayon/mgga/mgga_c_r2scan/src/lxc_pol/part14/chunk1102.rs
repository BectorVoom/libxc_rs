//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1102/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1102(t37541: f64, t37560: f64, t37568: f64, t38225: f64, t38228: f64, t38233: f64, t38244: f64, t38264: f64, t38267: f64, t38269: f64, t38281: f64, t38297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39094 = 0.487802396665200453e-2_f64 * t37541;
    let t39097 = 0.2439011983326002265e-2_f64 * t37560;
    let t39099 = 0.30487649791575028312e-3_f64 * t37568;
    let t39106 = 0.18292589874945016987e-2_f64 * t38225;
    let t39107 = 0.1299607316140891005e-4_f64 * t38228;
    let t39108 = 0.11709622077411463733e-2_f64 * t38233;
    let t39109 = 0.205201155180140685e-5_f64 * t38244;
    let t39113 = 0.30487649791575028312e-3_f64 * t38264;
    let t39114 = 0.18292589874945016987e-2_f64 * t38267;
    let t39115 = 0.487802396665200453e-2_f64 * t38269;
    let t39116 = 0.13010691197123848592e-3_f64 * t38281;
    let t39117 = 0.18292589874945016987e-2_f64 * t38297;
    (t39094, t39097, t39099, t39106, t39107, t39108, t39109, t39113, t39114, t39115, t39116, t39117)
}
