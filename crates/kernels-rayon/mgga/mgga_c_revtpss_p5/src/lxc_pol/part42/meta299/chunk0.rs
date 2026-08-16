//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1064/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1064(t221: f64, t346: f64, t68: f64, t345: f64, t245: f64, t3089: f64, t3088: f64, t3114: f64, t11223: f64, t225: f64, t366: f64, t1026: f64, t371: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11735 = t221 * t68 * t346;
    let t11737 = 5.0_f64 / 1296.0_f64 * t345 * t11735;
    let t11772 = t3089 * t245;
    let t11773 = t3088 * t11772;
    let t11774 = t3114 * t11773;
    let t11788 = t11223 * t225;
    let t11789 = t11788 * t366;
    let t11817 = t371 * t676 * t1026;
    (t11737, t11772, t11773, t11774, t11788, t11789, t11817)
}
