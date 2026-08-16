//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1639/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1639(t1860: f64, t23998: f64, t2031: f64, t22489: f64, t2032: f64, t22493: f64, t22519: f64, t22527: f64, t22531: f64, t22534: f64, t22537: f64, t22546: f64, t22549: f64, t23963: f64, t23968: f64, t23970: f64, t23973: f64, t23975: f64, t23978: f64, t23995: f64, t6486: f64, t6492: f64, t6495: f64, t7026: f64, t7035: f64) -> (f64, f64, f64) {
    let t23999 = t1860 * t23998;
    let t24001 = t2031 * t22489;
    let t24006 = 10.0_f64 * t23963 * t22546 + 80.0_f64 / 9.0_f64 * t23968 + 20.0_f64 / 3.0_f64 * t22549 * t23970 + 32.0_f64 / 9.0_f64 * t23973 - 10.0_f64 / 3.0_f64 * t23975 * t6492 - 16.0_f64 / 9.0_f64 * t23978 - 4.0_f64 / 3.0_f64 * t22519 * t2032 - 10.0_f64 / 3.0_f64 * t7026 * t22527 - 5.0_f64 / 3.0_f64 * t7026 * t22531 - 2.0_f64 / 3.0_f64 * t22534 * t2032 - 2.0_f64 / 3.0_f64 * t22537 * t2032 - 4.0_f64 / 3.0_f64 * t6495 * t7035 + t23995 + 2.0_f64 / 3.0_f64 * t6486 * t7035 - 16.0_f64 / 9.0_f64 * t23999 + t1860 * t24001 / 3.0_f64 + t22493 * t2032 / 3.0_f64;
    (t23999, t24001, t24006)
}
