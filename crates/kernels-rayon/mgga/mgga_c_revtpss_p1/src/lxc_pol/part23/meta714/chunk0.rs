//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2473/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2473(t3863: f64, t5567: f64, t3857: f64, t2608: f64, t512: f64, t5566: f64, t1856: f64, t9544: f64, t46975: f64, t46979: f64, t13597: f64, t2516: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48234 = 96.0_f64 * t3863 * t5567;
    let t48235 = t3857 * t5567;
    let t48236 = 60.0_f64 * t48235;
    let t48240 = t512 * t5566 * t2608;
    let t48241 = 3.0_f64 * t48240;
    let t48243 = t512 * t1856 * t9544;
    let t48244 = 240.0_f64 * t46975;
    let t48248 = 96.0_f64 * t46979;
    let t48255 = t13597 * t2516;
    (t48234, t48236, t48241, t48243, t48244, t48248, t48255)
}
