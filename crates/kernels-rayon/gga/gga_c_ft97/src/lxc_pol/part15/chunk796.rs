//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 796/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk796(t21639: f64, t762: f64, t242: f64, t1131: f64, t4635: f64, t2600: f64, t2599: f64, t1168: f64, t2607: f64, t2606: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21640 = t762 * t21639;
    let t21641 = t242 * t21640;
    let t21645 = t4635 * t1131;
    let t21646 = t2600 * t21645;
    let t21647 = t2599 * t21646;
    let t21650 = t4635 * t1168;
    let t21651 = t2607 * t21650;
    let t21652 = t2606 * t21651;
    let t21655 = t4635 * t992;
    (t21640, t21641, t21645, t21646, t21647, t21650, t21651, t21652, t21655)
}
