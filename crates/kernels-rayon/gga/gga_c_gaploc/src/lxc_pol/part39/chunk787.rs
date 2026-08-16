//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 787/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk787(t12079: f64, t901: f64, t2366: f64, t3689: f64, t2365: f64, t1429: f64, t12533: f64, t12536: f64, t12065: f64, t895: f64, t11986: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13789 = t12079 * t901;
    let t13791 = t2366 * t3689;
    let t13792 = t2365 * t13791;
    let t13793 = t1429 * t13792;
    let t13795 = 0.38342925953920749677e0_f64 * t12533;
    let t13796 = 0.38342925953920749677e0_f64 * t12536;
    let t13798 = t895 * t12065;
    let t13800 = t11986 * t874;
    (t13789, t13791, t13792, t13793, t13795, t13796, t13798, t13800)
}
