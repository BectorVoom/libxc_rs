//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1243/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1243(t13625: f64, t25082: f64, t32113: f64, t26088: f64, t531: f64, t2014: f64, t7238: f64, t25090: f64, t7235: f64, t25803: f64, t25802: f64, t7312: f64) -> (f64, f64, f64, f64, f64) {
    let t94355 = 18.0_f64 * t25082 * t32113 * t13625;
    let t94358 = t531 * t26088;
    let t94361 = 9.0_f64 * t2014 * t94358 * t7238;
    let t94369 = 9.0_f64 * t7235 * t25090;
    let t94371 = 3.0_f64 * t7235 * t25803;
    let t94374 = 3.0_f64 * t2014 * t7312 * t25802;
    (t94355, t94361, t94369, t94371, t94374)
}
