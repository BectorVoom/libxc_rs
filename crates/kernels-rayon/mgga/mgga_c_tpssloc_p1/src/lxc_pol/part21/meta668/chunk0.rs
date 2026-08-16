//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2469/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2469(t42341: f64, t44696: f64, t42344: f64, t483: f64, t1210: f64, t1174: f64, t3561: f64, t698: f64, t10471: f64, t44690: f64, t11727: f64, t44722: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44833 = t44696 * t42341;
    let t44834 = t483 * t42344;
    let t44836 = t44833 * t1210 * t44834;
    let t44847 = t1174 * t698 * t3561;
    let t44857 = t44690 * t10471;
    let t44858 = t44857 * t11727;
    let t44863 = t44833 * t44722 * t478 * t44834;
    (t44833, t44834, t44836, t44847, t44857, t44858, t44863)
}
