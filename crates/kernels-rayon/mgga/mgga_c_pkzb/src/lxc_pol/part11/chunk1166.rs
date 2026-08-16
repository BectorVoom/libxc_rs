//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1166/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1166(t10437: f64, t16089: f64, t444: f64, t1429: f64, t3329: f64, t8: f64, t3333: f64, t983: f64, t1430: f64, t2499: f64, t8657: f64, t10444: f64, t1435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28696 = t16089 * t10437 * t444;
    let t28700 = t3329 * t8 * t1429;
    let t28703 = t983 * t3333;
    let t28704 = t28703 * t444;
    let t28707 = t1430 * t3333;
    let t28710 = t2499 * t8657;
    let t28714 = t1435 * t10444 * t444;
    (t28696, t28700, t28704, t28707, t28710, t28714)
}
