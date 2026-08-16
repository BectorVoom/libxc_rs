//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1060/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1060(t13093: f64, t13099: f64, t13111: f64, t13138: f64, t225: f64, t68: f64, t822: f64, t1484: f64, t1891: f64, t2379: f64, t4119: f64, t845: f64) -> (f64, f64, f64, f64) {
    let t13141 = (t13093 + t13099 + t13111 + t13138) * t225;
    let t13151 = t822 * t68;
    let t13156 = t1891 * t1484;
    let t13157 = t13156 * t2379;
    let t13160 = t845 * t4119;
    (t13141, t13151, t13157, t13160)
}
