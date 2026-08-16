//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1043/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1043(t25610: f64, t31949: f64, t1035: f64, t1061: f64, t1078: f64, t31897: f64, t3173: f64, t32000: f64, t8513: f64, t93469: f64, t11627: f64, t3148: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120223 = t25610 * t31949;
    let t120237 = t1078 * t1035 * t1061;
    let t120238 = t31897 * t120237;
    let t120244 = t32000 * t3173;
    let t120248 = t8513 * t93469;
    let t120251 = t120248 * t1078 * t11627 * t3148;
    (t120223, t120237, t120238, t120244, t120248, t120251)
}
