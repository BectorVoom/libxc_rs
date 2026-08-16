//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1876/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1876(t22766: f64, t22780: f64, t22798: f64, t22805: f64, t22820: f64, t22826: f64, t26231: f64, t26234: f64, t26236: f64, t26238: f64, t26240: f64, t26246: f64, t26249: f64, t26251: f64, t26280: f64, t26286: f64, t26290: f64, t26293: f64, t26295: f64, t26299: f64, t26303: f64, t26326: f64) -> f64 {
    let t26328 = 7.0_f64 / 2304.0_f64 * t26231 - t26234 / 1536.0_f64 - t26236 / 1536.0_f64 - t26238 / 1536.0_f64 + 5.0_f64 / 384.0_f64 * t26240 + 7.0_f64 / 2304.0_f64 * t22766 + 0.33643963411783659045e-4_f64 * t26246 + t26249 / 1536.0_f64 - 7.0_f64 / 2304.0_f64 * t26251 + 0.14130464632949136799e-2_f64 * t22780 + t26280 + 7.0_f64 / 144.0_f64 * t22798 + 0.84782787797694820794e-2_f64 * t22805 - t22820 + t22826 + t26286 / 16.0_f64 + 0.84782787797694820792e-2_f64 * t26290 - 0.20186378047070195427e-3_f64 * t26293 + 0.14130464632949136799e-2_f64 * t26295 + 0.12111826828242117256e-2_f64 * t26299 + 0.12111826828242117256e-2_f64 * t26303 + t26326;
    t26328
}
