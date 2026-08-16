//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1815/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1815(t13975: f64, t4594: f64, t4582: f64, t3120: f64, t3131: f64, t4593: f64, t10482: f64, t3040: f64, t3132: f64, t3069: f64, t4669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13976 = t13975 * t4594;
    let t13977 = t4582 * t13976;
    let t13980 = t3131 * t3120;
    let t13981 = t4593 * t13980;
    let t13982 = t4582 * t13981;
    let t13985 = t10482 * t3040;
    let t13986 = t4593 * t13985;
    let t13987 = t4582 * t13986;
    let t13990 = t4593 * t3132;
    let t13991 = t4582 * t13990;
    let t13995 = t4669 * t3069;
    (t13976, t13977, t13980, t13981, t13982, t13985, t13986, t13987, t13990, t13991, t13995)
}
