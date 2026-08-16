//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1276/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1276(t1692: f64, t1812: f64, t1989: f64, t1288: f64, t18728: f64, t18807: f64, t19672: f64, t19678: f64, t19681: f64, t19685: f64, t19810: f64, t19819: f64, t19821: f64, t19825: f64, t19829: f64, t19836: f64, t20417: f64, t20510: f64, t20514: f64, t20526: f64, t2439: f64, t30: f64, t5539: f64, t5591: f64, t580: f64, t5849: f64, t5853: f64, t6120: f64, t6153: f64, t6354: f64) -> (f64, f64) {
    let t20544 = t1692 * t1812 * t1989;
    let t20545 = 3.0_f64 * t20417 * t19672 + 3.0_f64 / 2.0_f64 * t2439 * t5849 * t6120 - 3.0_f64 / 2.0_f64 * t18728 * t19678 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t19681 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t19685 + 3.0_f64 / 2.0_f64 * t2439 * t6354 * t5539 + t1692 * t20510 * t30 / 2.0_f64 - t1692 * t20514 * t5591 / 2.0_f64 + t1692 * t6354 * t580 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t18728 * t19810 - t1692 * t18807 * t6153 / 2.0_f64 + t20526 * t19819 - t1692 * t5853 * t19821 / 2.0_f64 - t1692 * t5853 * t19825 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t19829 + t1692 * t5849 * t1288 / 2.0_f64 - t1692 * t5853 * t19836 / 2.0_f64 + t20544;
    (t20544, t20545)
}
