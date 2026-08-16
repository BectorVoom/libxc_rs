//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2692/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2692(t4772: f64, t4975: f64, t19462: f64, t3286: f64, t3298: f64, t6235: f64, t3316: f64, t1086: f64, t19856: f64, t16543: f64, t4746: f64, t1647: f64, t16551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67668 = t4975 * t4772;
    let t67714 = t19462 * t3286;
    let t67725 = t6235 * t3298;
    let t67790 = t6235 * t3316;
    let t67825 = t19856 * t1086;
    let t67927 = t4746 * t16543;
    let t67969 = t1647 * t16551;
    (t67668, t67714, t67725, t67790, t67825, t67927, t67969)
}
