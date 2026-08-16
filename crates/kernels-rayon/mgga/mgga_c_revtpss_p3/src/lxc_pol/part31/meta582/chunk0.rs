//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2002/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2002(t11735: f64, t1968: f64, t11772: f64, t25515: f64, t3114: f64, t3223: f64, t7131: f64, t11273: f64, t25504: f64, t25508: f64, t11263: f64, t7122: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93750 = 5.0_f64 / 1296.0_f64 * t1968 * t11735;
    let t93751 = t25515 * t11772;
    let t93752 = t3114 * t93751;
    let t93764 = t3223 * t7131;
    let t93783 = t11273 * t25504;
    let t93796 = t11273 * t25508;
    let t93801 = t7122 * t11263;
    (t93750, t93751, t93752, t93764, t93783, t93796, t93801)
}
