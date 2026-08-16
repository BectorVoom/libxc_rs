//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1001/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1001(t10764: f64, t226: f64, t773: f64, t774: f64, t10661: f64, t10664: f64, t10669: f64, t10674: f64, t10678: f64, t10679: f64, t771: f64, t797: f64, t8177: f64, t8179: f64, t8188: f64, t8204: f64, t8205: f64, t8287: f64) -> (f64, f64, f64) {
    let t10765 = t10764 * t226;
    let t10767 = t773 * t774 * t10765;
    let t10772 = -35.0_f64 / 108.0_f64 * t8177 + 7.0_f64 / 144.0_f64 * t8179 - t8188 - t10661 + 5.0_f64 / 384.0_f64 * t797 * t10664 + 5.0_f64 / 768.0_f64 * t797 * t10669 - 5.0_f64 / 128.0_f64 * t797 * t10674 + t10678 - 119.0_f64 / 13824.0_f64 * t10679 - t771 * t10767 / 3072.0_f64 - t8204 + 7.0_f64 / 4608.0_f64 * t8205 - 119.0_f64 / 6912.0_f64 * t8287;
    (t10765, t10767, t10772)
}
