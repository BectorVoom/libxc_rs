//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 978/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk978(t2141: f64, t77698: f64, t75638: f64, t75640: f64, t75644: f64, t1986: f64, t2469: f64, t7720: f64, t71366: f64, t9222: f64, t71154: f64, t8571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77699 = t77698 * t2141;
    let t77700 = 0.13637330827122670864e-1_f64 * t77699;
    let t77703 = 0.14967802127329760705e-1_f64 * t75638;
    let t77704 = 0.10227998120342003148e-1_f64 * t75640;
    let t77705 = 0.10227998120342003148e-1_f64 * t75644;
    let t77711 = t1986 * t2469;
    let t77712 = t7720 * t77711;
    let t77713 = 0.85129199786595678796e-5_f64 * t77712;
    let t77714 = t9222 * t71366;
    let t77715 = 0.53205749866622299248e-5_f64 * t77714;
    let t77716 = t8571 * t71154;
    (t77700, t77703, t77704, t77705, t77713, t77715, t77716)
}
