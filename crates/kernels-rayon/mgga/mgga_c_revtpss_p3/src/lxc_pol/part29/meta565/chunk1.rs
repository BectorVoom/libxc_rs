//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1911/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1911(t786: f64, t97961: f64, t1444: f64, t5675: f64, t25898: f64, t98040: f64, t1907: f64, t3889: f64, t25081: f64, t7897: f64, t1518: f64, t2319: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98308 = t786 * t97961;
    let t98362 = t5675 * t1444;
    let t98380 = t98040 * t25898;
    let t98436 = t1907 * t3889;
    let t98450 = t7897 * t25081;
    let t98484 = t2319 * t1518;
    (t98308, t98362, t98380, t98436, t98450, t98484)
}
