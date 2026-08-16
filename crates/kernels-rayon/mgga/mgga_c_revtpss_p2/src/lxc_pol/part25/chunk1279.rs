//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1279/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1279(t11937: f64, t25500: f64, t1024: f64, t25553: f64, t25495: f64, t3215: f64, t11817: f64, t7117: f64, t3223: f64, t7125: f64, t11940: f64, t1972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93713 = t25500 * t11937;
    let t93715 = t1024 * t25553;
    let t93718 = t25495 * t3215;
    let t93720 = t7117 * t11817;
    let t93722 = t3223 * t7125;
    let t93725 = t11940 * t1972;
    (t93713, t93715, t93718, t93720, t93722, t93725)
}
