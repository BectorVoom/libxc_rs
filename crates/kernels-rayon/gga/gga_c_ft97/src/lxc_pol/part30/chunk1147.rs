//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1147/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1147(t1466: f64, t35794: f64, t681: f64, t35801: f64, t36063: f64, t44351: f64, t1476: f64, t7129: f64, t143017: f64, t6967: f64, t1506: f64, t7021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t153681 = t1466 * t681 * t35794;
    let t153684 = t1466 * t681 * t35801;
    let t153687 = t44351 * t36063;
    let t153689 = t1476 * t7129;
    let t153696 = t143017 * t6967;
    let t153698 = t7021 * t1506;
    (t153681, t153684, t153687, t153689, t153696, t153698)
}
