//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2184/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2184(t1174: f64, t3561: f64, t698: f64, t11738: f64, t11739: f64, t248: f64, t3570: f64, t10471: f64, t44690: f64, t11727: f64, t44722: f64, t44833: f64, t44834: f64, t478: f64) -> (f64, f64, f64, f64, f64) {
    let t44847 = t1174 * t698 * t3561;
    let t44851 = t11738 * t248 * t3570 * t11739;
    let t44857 = t44690 * t10471;
    let t44858 = t44857 * t11727;
    let t44863 = t44833 * t44722 * t478 * t44834;
    (t44847, t44851, t44857, t44858, t44863)
}
