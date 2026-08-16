//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1036/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1036(t11381: f64, t11416: f64, t11444: f64, t11481: f64, t158: f64, t1255: f64, t3909: f64, t6546: f64, t3254: f64, t3928: f64, t11345: f64, t5728: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11483 = t11381 + t11416 + t11444 + t11481;
    let t11484 = t11483 * t158;
    let t11493 = t3909 * t1255;
    let t11494 = t6546 * t11493;
    let t11497 = t3254 * t3928;
    let t11500 = t11345 * t5728;
    (t11483, t11484, t11493, t11494, t11497, t11500)
}
