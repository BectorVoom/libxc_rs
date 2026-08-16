//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1628/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1628(t20795: f64, t5352: f64, t3720: f64, t3153: f64, t6622: f64, t5341: f64, t5333: f64, t1263: f64, t6587: f64, t1122: f64, t1042: f64, t3172: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20796 = t20795 * t5352;
    let t20797 = t3720 * t20796;
    let t20800 = t6622 * t3153;
    let t20801 = t20800 * t5341;
    let t20802 = t3720 * t20801;
    let t20805 = t20800 * t5333;
    let t20806 = t3720 * t20805;
    let t20809 = t1263 * t6587;
    let t20810 = t20809 * t1122;
    let t20811 = t1042 * t20810;
    let t20816 = t3172 * t6624;
    (t20797, t20800, t20802, t20806, t20811, t20816)
}
