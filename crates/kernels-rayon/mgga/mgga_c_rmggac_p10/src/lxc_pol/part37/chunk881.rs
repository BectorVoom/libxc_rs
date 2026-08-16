//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 881/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk881(t13862: f64, t3120: f64, t8581: f64, t14107: f64, t15280: f64, t21708: f64, t68422: f64, t9212: f64, t21714: f64, t9217: f64, t14125: f64, t9105: f64) -> (f64, f64, f64, f64, f64) {
    let t75792 = t3120 * t13862 * t8581;
    let t75794 = t15280 * t14107;
    let t75797 = t21708 * t68422 * t9212;
    let t75800 = t21708 * t21714 * t9217;
    let t75803 = t21708 * t14125 * t9105;
    (t75792, t75794, t75797, t75800, t75803)
}
