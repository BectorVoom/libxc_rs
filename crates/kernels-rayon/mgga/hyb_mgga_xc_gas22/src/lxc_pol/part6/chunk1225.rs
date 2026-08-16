//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1225/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1225(t1291: f64, t136: f64, t2986: f64, t677: f64, t8223: f64, t684: f64, t8184: f64, t8457: f64, t8453: f64, t1319: f64, t1240: f64, t6229: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23975 = t136 * t2986 * t1291;
    let t23977 = t677 * t8223;
    let t23985 = t684 * t8184 * t8457;
    let t23987 = t677 * t8453;
    let t23990 = t136 * t2986 * t1319;
    let t23992 = t1240 * t6229;
    (t23975, t23977, t23985, t23987, t23990, t23992)
}
