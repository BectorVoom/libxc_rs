//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 804/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk804(t9798: f64, t9860: f64, t157: f64, t153: f64, t181: f64, t686: f64, t781: f64, t756: f64, t2371: f64, t677: f64, t2374: f64, t2535: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9861 = t9798 + t9860;
    let t9862 = t157 * t9861;
    let t9863 = t153 * t9862;
    let t9874 = t686 * t781 * t181;
    let t9876 = 0.56968947174242584612e-3_f64 * t756 * t9874;
    let t9882 = t677 * t2371;
    let t9884 = 0.32530743900905219526e-1_f64 * t2374 * t9882;
    let t9885 = t677 * t2535;
    (t9861, t9862, t9863, t9874, t9876, t9882, t9884, t9885)
}
