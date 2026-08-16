//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 751/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk751(t4872: f64, t8680: f64, t925: f64, t1073: f64, t2266: f64, t4458: f64, t4462: f64, t4883: f64, t20022: f64, t2259: f64, t72: f64, t20039: f64, t3621: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21048 = t8680 * t925 * t4872;
    let t21052 = t2266 * t4458 * t1073;
    let t21056 = t2266 * t4462 * t1073;
    let t21058 = t925 * t4883;
    let t21059 = t2266 * t21058;
    let t21062 = t72 * t2259 * t20022;
    let t21064 = t3621 * t20039;
    (t21048, t21052, t21056, t21058, t21059, t21062, t21064)
}
