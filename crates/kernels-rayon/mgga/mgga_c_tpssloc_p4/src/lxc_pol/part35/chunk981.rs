//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 981/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk981(t185: f64, t20217: f64, t707: f64, t13115: f64, t5499: f64, t20777: f64, t20815: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64, t9894: f64) -> (f64, f64, f64) {
    let t20816 = t185 * t20217;
    let t20818 = 4.0_f64 * t707 * t20816;
    let t20820 = 36.0_f64 * t13115 * t5499;
    let t20821 = -t9876 - t9820 - t9824 - t9884 + t9887 + t9890 - t20777 + t20815 + t20818 - t9894 + t20820;
    (t20818, t20820, t20821)
}
