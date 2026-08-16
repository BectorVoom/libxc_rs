//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 796/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk796(t205: f64, t9558: f64, t210: f64, t214: f64, t9458: f64, t213: f64, t776: f64, t221: f64, t2553: f64, t59: f64, t8705: f64) -> (f64, f64, f64, f64) {
    let t9559 = t205 * t9558;
    let t9561 = t210 * t214 * t9458;
    let t9564 = t213 * t776;
    let t9566 = t221 * t9564 * t2553;
    let t9569 = t59 * t8705;
    (t9559, t9561, t9566, t9569)
}
