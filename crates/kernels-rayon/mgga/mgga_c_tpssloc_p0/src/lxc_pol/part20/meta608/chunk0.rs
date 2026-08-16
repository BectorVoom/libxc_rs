//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2193/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2193(t3575: f64, t42386: f64, t11888: f64, t11914: f64, t11784: f64, t820: f64, t11669: f64, t3577: f64, t11779: f64, t11677: f64, t11907: f64, t11769: f64, t13969: f64, t3515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45113 = t3575 * t42386;
    let t45114 = t11888 * t45113;
    let t45119 = t11914 * t45113;
    let t45124 = t820 * t11784;
    let t45126 = t3577 * t45124 * t11669;
    let t45128 = t820 * t11779;
    let t45134 = t11907 * t11677;
    let t45148 = t3515 * t13969 * t11769;
    (t45113, t45114, t45119, t45124, t45126, t45128, t45134, t45148)
}
