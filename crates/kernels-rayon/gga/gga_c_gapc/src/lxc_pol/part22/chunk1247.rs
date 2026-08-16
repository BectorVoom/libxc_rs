//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1247/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1247(t11474: f64, t8880: f64, t3076: f64, t34714: f64, t11455: f64, t1453: f64, t505: f64, t5526: f64, t674: f64, t34503: f64, t9256: f64, t26007: f64, t3708: f64, t9304: f64) -> (f64, f64, f64, f64, f64) {
    let t34832 = t11474 * t8880;
    let t34834 = t34714 * t3076;
    let t34839 = t11455 * t1453 * t505 * t674 * t5526;
    let t34846 = t34503 * t9256;
    let t34849 = t9304 * t3708 * t26007;
    (t34832, t34834, t34839, t34846, t34849)
}
