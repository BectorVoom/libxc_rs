//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1152/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1152(t2435: f64, t28448: f64, t103431: f64, t25375: f64, t103421: f64, t7058: f64, t11064: f64, t8019: f64, t5891: f64, t94978: f64, t25823: f64, t5915: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103490 = t2435 * t28448;
    let t103521 = t25375 * t103431;
    let t103547 = t7058 * t103421;
    let t103586 = t8019 * t11064;
    let t105870 = t94978 * t5891;
    let t105878 = t25823 * t5915;
    (t103490, t103521, t103547, t103586, t105870, t105878)
}
