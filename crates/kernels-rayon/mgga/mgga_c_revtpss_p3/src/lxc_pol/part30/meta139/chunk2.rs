//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 755/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk755(t3059: f64, t996: f64, t1071: f64, t994: f64, t1096: f64, t999: f64, t1079: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3060 = t996 * t3059;
    let t3063 = t994 * t1071;
    let t3066 = t999 * t1096;
    let t3067 = t1079 * t3066;
    let t3070 = 0.19755555555555555556e-1_f64 * t2846;
    let t3075 = t3070 + 0.9877777777777777778e-2_f64 * t2848 - 0.9877777777777777778e-2_f64 * t2855 + 0.29633333333333333334e-1_f64 * t2860 - 0.14816666666666666667e-1_f64 * t2864;
    (t3060, t3063, t3066, t3067, t3070, t3075)
}
