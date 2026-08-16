//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1240/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1240(t128302: f64, t2055: f64, t28042: f64, t93: f64, t34321: f64, t7373: f64, t32392: f64, t7983: f64, t32655: f64, t28683: f64, t8692: f64, t32385: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t128358 = t128302 * t2055;
    let t128359 = t93 * t28042;
    let t128360 = t128359 * t2055;
    let t128361 = t34321 * t7373;
    let t128362 = t32392 * t7983;
    let t128363 = t32655 * t7983;
    let t128367 = 2.0_f64 * t8692 * t28683;
    let t128483 = 2.0_f64 * t7732 * t32385;
    (t128358, t128360, t128361, t128362, t128363, t128367, t128483)
}
