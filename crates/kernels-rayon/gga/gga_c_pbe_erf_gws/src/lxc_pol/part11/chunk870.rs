//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 870/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk870(t13086: f64, t376: f64, t338: f64, t353: f64, t1144: f64, t3896: f64, t1105: f64, t3721: f64, t2409: f64, t3067: f64, t3737: f64, t13290: f64, t829: f64, t830: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13639 = t376 * t13086;
    let t13641 = t338 * t353 * t13639;
    let t13645 = t338 * t1144 * t3896;
    let t13648 = t3721 * t1105;
    let t13650 = t2409 * t3067 * t13648;
    let t13656 = t338 * t1144 * t3737;
    let t13662 = t829 * t830 * t831 * t13290;
    (t13639, t13641, t13645, t13648, t13650, t13656, t13662)
}
