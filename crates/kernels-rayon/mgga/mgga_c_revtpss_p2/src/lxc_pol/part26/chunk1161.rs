//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1161/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1161(t26205: f64, t6963: f64, t45972: f64, t7342: f64, t10309: f64, t26178: f64, t25159: f64, t606: f64, t68: f64, t2047: f64, t92569: f64, t2048: f64, t25114: f64, t25120: f64, t26175: f64, t26187: f64, t603: f64, t7343: f64, t7352: f64, t92568: f64, t92581: f64, t92658: f64, t92662: f64, t92672: f64, t92674: f64, t92692: f64, t92711: f64) -> f64 {
    let t95314 = t6963 * t26205;
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    let t95320 = t95319 * t25159;
    let t95334 = t606 * t68;
    let t95340 = t2047 * t92569;
    let t95343 = -176.0_f64 / 9.0_f64 * t95314 - 70.0_f64 * t95316 * t92692 - 80.0_f64 * t95320 - 2.0_f64 / 3.0_f64 * t92674 * t2048 - 2.0_f64 * t25120 * t7352 - 5.0_f64 * t26187 * t25114 - 2.0_f64 * t92711 * t2048 - 5.0_f64 * t7343 * t92658 - 5.0_f64 / 3.0_f64 * t7343 * t92662 - 2.0_f64 * t603 * t95334 * t92672 + 30.0_f64 * t26175 * t92581 - 60.0_f64 * t92568 * t95340;
    t95343
}
