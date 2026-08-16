//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1341/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1341(t1799: f64, t2105: f64, t5815: f64, t645: f64, t5935: f64, t9895: f64, t1692: f64, t1989: f64, t5849: f64, t18728: f64, t63785: f64, t17921: f64, t17934: f64, t1812: f64, t18803: f64, t18807: f64, t19825: f64, t1991: f64, t20510: f64, t2439: f64, t3552: f64, t5539: f64, t5853: f64, t6120: f64, t6354: f64, t63766: f64, t63823: f64, t63873: f64, t63877: f64, t63885: f64, t64273: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66195 = t2105 * t1799;
    let t66199 = t645 * t5815;
    let t66217 = t5935 * t9895;
    let t66235 = 2.0_f64 * t1692 * t5849 * t1989;
    let t66262 = 6.0_f64 * t18728 * t63785;
    let t66266 = 3.0_f64 * t2439 * t6354 * t17934 - 3.0_f64 * t18728 * t63766 + t66235 + 3.0_f64 * t2439 * t20510 * t5539 + 6.0_f64 * t18728 * t63885 - t1692 * t18807 * t19825 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t63873 + t1692 * t6354 * t1991 / 2.0_f64 + 3.0_f64 * t2439 * t1812 * t63877 - t1692 * t5853 * t64273 / 2.0_f64 + 3.0_f64 * t3552 * t1812 * t63823 + 3.0_f64 * t3552 * t6354 * t17921 + t66262 + 3.0_f64 / 2.0_f64 * t2439 * t18803 * t6120;
    (t66195, t66199, t66217, t66235, t66262, t66266)
}
