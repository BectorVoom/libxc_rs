//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1496/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1496(t22352: f64, t2435: f64, t2289: f64, t5916: f64, t5892: f64, t25048: f64, t575: f64, t22590: f64, t625: f64, t22593: f64, t22629: f64, t116: f64, t22746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t75274 = t2435 * t22352;
    let t75540 = t2289 * t5916;
    let t75639 = t2289 * t5892;
    let t75808 = t25048 * t575;
    let t75822 = t625 * t22590;
    let t75831 = t625 * t22593;
    let t75843 = t625 * t22629;
    let t75941 = t22746 * t116;
    (t75274, t75540, t75639, t75808, t75822, t75831, t75843, t75941)
}
