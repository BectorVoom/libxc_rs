//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1467/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1467(t120: f64, t5527: f64, t829: f64, t9646: f64, t5544: f64, t2645: f64, t16839: f64, t2647: f64, t13177: f64, t13251: f64, t13260: f64, t13275: f64, t13277: f64, t13280: f64, t13287: f64, t13320: f64, t13330: f64, t1512: f64, t16872: f64, t16877: f64, t16879: f64, t16888: f64, t16893: f64, t2643: f64, t4167: f64, t4178: f64, t4191: f64, t4236: f64, t4240: f64, t4250: f64, t831: f64) -> (f64, f64, f64, f64) {
    let t16896 = t120 * t5527;
    let t16898 = t9646 * t16896 * t829;
    let t16901 = t120 * t5544;
    let t16903 = t2645 * t16901 * t829;
    let t16907 = t2645 * t16839 * t2647;
    let t16910 = -t4167 * t4236 / 1536.0_f64 - t16872 * t831 / 3072.0_f64 - t13177 * t1512 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t16877 - 7.0_f64 / 2304.0_f64 * t16879 - t13260 + t13275 + t13277 + t13280 - t13287 + t13251 * t4191 / 384.0_f64 - t13251 * t4240 / 1536.0_f64 + t13251 * t4250 / 384.0_f64 - 5.0_f64 / 384.0_f64 * t2643 * t16888 + t4178 * t16893 / 1536.0_f64 - 5.0_f64 / 768.0_f64 * t2643 * t16898 + t2643 * t16903 / 768.0_f64 + t13320 - t13330 + t2643 * t16907 / 768.0_f64;
    (t16898, t16903, t16907, t16910)
}
