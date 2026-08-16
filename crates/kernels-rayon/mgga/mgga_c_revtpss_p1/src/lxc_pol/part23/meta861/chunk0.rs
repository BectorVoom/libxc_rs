//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2751/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2751(t1204: f64, t6695: f64, t1276: f64, t6573: f64, t12587: f64, t6748: f64, t21635: f64, t3801: f64, t3857: f64, t6801: f64, t3860: f64, t123: f64, t2630: f64, t6800: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t73222 = t1204 * t6695;
    let t73236 = t1276 * t6573;
    let t73252 = t6748 * t12587;
    let t73273 = t21635 * t3801;
    let t73321 = t3857 * t6801;
    let t73329 = t3860 * t6801;
    let t73341 = t6800 * t123 * t2630;
    (t73222, t73236, t73252, t73273, t73321, t73329, t73341)
}
