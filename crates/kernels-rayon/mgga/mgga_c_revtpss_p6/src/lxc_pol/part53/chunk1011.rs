//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1011/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1011(t32103: f64, t651: f64, t2322: f64, t8461: f64, t4254: f64, t1310: f64, t8460: f64, t4147: f64, t7311: f64, t2034: f64, t2014: f64, t7315: f64, t8595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32104 = t651 * t32103;
    let t32106 = t2322 * t8461;
    let t32107 = 2.0_f64 * t32106;
    let t32108 = t4254 * t8461;
    let t32109 = 2.0_f64 * t32108;
    let t32110 = t1310 * t8460;
    let t32111 = t651 * t32110;
    let t32112 = 2.0_f64 * t32111;
    let t32113 = t4147 * t7311;
    let t32114 = t2034 * t32113;
    let t32116 = 2.0_f64 * t2014 * t32114;
    let t32117 = t8595 * t7315;
    (t32104, t32107, t32109, t32110, t32112, t32114, t32116, t32117)
}
