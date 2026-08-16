//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 926/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk926(t531: f64, t8594: f64, t7238: f64, t2014: f64, t7235: f64, t8600: f64, t2322: f64, t8461: f64, t4254: f64, t1310: f64, t8460: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32098 = t531 * t8594;
    let t32099 = t32098 * t7238;
    let t32101 = 3.0_f64 * t2014 * t32099;
    let t32102 = t7235 * t8600;
    let t32106 = t2322 * t8461;
    let t32107 = 2.0_f64 * t32106;
    let t32108 = t4254 * t8461;
    let t32109 = 2.0_f64 * t32108;
    let t32110 = t1310 * t8460;
    let t32111 = t651 * t32110;
    (t32098, t32099, t32101, t32102, t32107, t32109, t32110, t32111)
}
