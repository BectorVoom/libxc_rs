//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3193/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3193(t3584: f64, t5352: f64, t3568: f64, t3603: f64, t1248: f64, t1247: f64, t1796: f64, t42994: f64, t1261: f64, t17231: f64, t3172: f64, t1250: f64) -> (f64, f64, f64, f64, f64) {
    let t58798 = t5352 * t3584;
    let t58803 = t3603 * t3568;
    let t58804 = t58803 * t1248;
    let t58824 = t1247 * t42994 * t1796;
    let t58827 = t1261 * t3172 * t17231;
    let t58831 = t1250 * t3568;
    (t58798, t58804, t58824, t58827, t58831)
}
